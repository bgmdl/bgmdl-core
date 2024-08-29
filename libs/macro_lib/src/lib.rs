extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use std::fs;

#[proc_macro]
pub fn pub_files(input: TokenStream) -> TokenStream {
    let d = parse_macro_input!(input as syn::LitStr).value();
    let handlers_path = format!("src/{}", d);
    let mut service_calls = Vec::new();
    for entry in fs::read_dir(handlers_path.as_str()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_stem() {
                if let Some(module_name) = filename.to_str() {
                    if module_name == "mod" {
                        continue;
                    }
                    let mod_ident = syn::Ident::new(module_name, proc_macro2::Span::call_site());
                    // mod_statements.push_str(&format!("mod {};\n", module_name));
                    service_calls.push(quote!{pub mod #mod_ident;});
                }
            }
        }
    }

    let expanded = quote! {
        #(#service_calls)*
    };

    TokenStream::from(expanded)
}

#[proc_macro]
pub fn generate_services(_input: TokenStream) -> TokenStream {
    let handlers_path = "src/handler";
    let mut service_calls = Vec::new();

    for entry in fs::read_dir(handlers_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_file() {
            if let Some(filename) = path.file_stem() {
                if let Some(module_name) = filename.to_str() {
                    if module_name == "mod" {
                        continue;
                    }
                    println!("Build module: {}", module_name);
                    let mod_ident = syn::Ident::new(module_name, proc_macro2::Span::call_site());
                    service_calls.push(quote!{.service(handler::#mod_ident::service())});
                }
            }
        }
    }

    let expanded = quote! {
        HttpServer::new(|| {
            App::new()
            #(#service_calls)*
            .app_data(web::JsonConfig::default().error_handler(|err, _req| {
                error::InternalError::from_response(
                    "",
                    HttpResponse::BadRequest()
                        .content_type("application/json")
                        .body(Json!{"code": -1, "msg": err.to_string()})
                ).into()
            }))
        })
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn perm(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let perm = parse_macro_input!(attr as syn::LitStr);

    let func_name = &input.sig.ident;
    let func_args = &input.sig.inputs;
    let func_body = &input.block;
    let func_return = &input.sig.output;

    let expanded = quote! {
        async fn #func_name(req: HttpRequest, #func_args) #func_return {
            // 检查用户权限
            // get user name
            let has_permission = check_user_permission(1, #perm).await;
            if !has_permission {
                return Ok(Json! {
                    "code": -1,
                    "msg": "No permission."
                });
            }

            #func_body
        }
    };

    TokenStream::from(expanded)
}
// 你的宏定义
#[proc_macro]
pub fn generate_commands(_item: TokenStream) -> TokenStream {
    // 目标文件夹
    let command_dir = "src/command";
    // 初始化子命令和运行逻辑向量
    let mut subcommands = Vec::new();
    let mut run_functions = Vec::new();

    // 遍历目标文件夹中的文件
    for entry in fs::read_dir(command_dir).expect("Failed to read command directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.is_file() && path.extension().unwrap_or_default() == "rs" && path.file_name() != Some("mod.rs".as_ref()) {
            // 读取文件内容
            let content = fs::read_to_string(&path).expect("Failed to read file");
            println!("Read file: {:?}", path);
            // 解析文件内容，提取相关信息
            if let Some((command, run_function)) = parse_command_and_run_from_file(&content, path.file_stem().unwrap().to_str().unwrap()) {
                subcommands.push(command);
                run_functions.push(run_function);
                println!("Parsed file: {:?}", path);
            }
        }
    }

    // 构造 clap::Command 代码
    let subcommands_code = subcommands.into_iter().map(|cmd| {
        quote! {
            .subcommand(#cmd)
        }
    });

    // 构造命令匹配和函数调用代码
    let match_arms = run_functions.into_iter().map(|(cmd_name, run_function_call)| {
        quote! {
            (#cmd_name, sub_m) => {
                #run_function_call
            }
        }
    });

    let expanded = quote! {
        pub fn build_cli() -> clap::Command {
            clap::Command::new("bangumidownload")
                #(#subcommands_code)*
        }

        pub fn execute_command() {
            let matches = build_cli().get_matches();

            match matches.subcommand().expect("clap will enforce a subcommand is provided") {
                #(#match_arms,)*
                (_,_) => {}
            }
        }
    };

    TokenStream::from(expanded)
}

fn parse_command_and_run_from_file(content: &str, path: &str) -> Option<(proc_macro2::TokenStream, (proc_macro2::TokenStream, proc_macro2::TokenStream))> {
    let router_line = content.lines().find(|line| line.starts_with("//! router:"))?;
    let description_line = content.lines().find(|line| line.starts_with("//! description:"))?;
    let run_line = content.lines().find(|line| line.contains("pub fn run("))?;
    let command_name = router_line.trim_start_matches("//! router:").trim();
    let description = description_line.trim_start_matches("//! description:").trim();
    let mut options = Vec::new();
    for line in content.lines() {
        if line.starts_with("//! --") {
            let option_def = line.trim_start_matches("//! ").trim();
            let parts: Vec<&str> = option_def.split(',').collect();
            if parts.len() == 2 {
                let option_args = parts[0].split_whitespace().collect::<Vec<&str>>();
                let help_message = parts[1].trim();
                if option_args.len() >= 3 {
                    let option_name = option_args[0].trim_start_matches("--");
                    let short_flag = option_args[1].trim_start_matches('-');
                    let value_name = option_args[2].trim_start_matches('<').trim_end_matches('>');
                    let short_flag_char = short_flag.chars().next().unwrap();
                    options.push(quote! {
                        .arg(clap::Arg::new(#option_name)
                            .short(#short_flag_char)
                            .long(#option_name)
                            .value_name(#value_name)
                            .help(#help_message))
                    });
                }
            }
        }
    }
    let run_fn_signature = run_line
        .trim()
        .trim_start_matches("pub fn run(")
        .trim_end_matches(") {");
    let run_fn_args: Vec<&str> = run_fn_signature.split(',').collect();
    let run_fn_call = run_fn_args.iter().map(|arg| {
        let arg_name = arg.split(':').next().unwrap().trim();
        let type_d = arg.split(':').nth(1).unwrap().trim();
        let type_d_ident = syn::Ident::new(type_d, proc_macro2::Span::call_site());
        quote! { *sub_m.get_one::<#type_d_ident>(#arg_name).unwrap() }
    });
    let func_ident = syn::Ident::new(path, proc_macro2::Span::call_site());
    Some((
        quote! {
            clap::Command::new(#command_name)
                .about(#description)
                #(#options)*
        },
        (
            quote! { #command_name },
            quote! { 
                command::#func_ident::run(#(#run_fn_call),*); 
            }
        ),
    ))
}
