extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use std::fs;

/* #[proc_macro]
pub fn pub_handler(_input: TokenStream) -> TokenStream {
    let handlers_path = format!("src/handler");
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
        } else if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            let mod_ident = syn::Ident::new(dir_name, proc_macro2::Span::call_site());
            service_calls.push(quote!{pub mod #mod_ident;});
        }
    }
    let expanded = quote! {
        #(#service_calls)*
    };

    TokenStream::from(expanded)
}
*/

#[proc_macro]
pub fn generate_services(_input: TokenStream) -> TokenStream {
    let handlers_path = "src/handler";
    let mut service_calls = Vec::new();
    let data = fs::read_dir(handlers_path).unwrap();
    let mut sorted_entries: Vec<_> = data
        .filter_map(|entry| entry.ok())
        .collect();
    sorted_entries.sort_by_key(|entry| entry.path());
    for entry in sorted_entries {
        let path = entry.path();
        if path.is_file() {
            if let Some(filename) = path.file_stem() {
                if let Some(module_name) = filename.to_str() {
                    if module_name == "mod" {
                        continue;
                    }
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

#[proc_macro]
pub fn generate_commands(_item: TokenStream) -> TokenStream {
    let command_dir = "src/command";
    let mut subcommands = Vec::new();
    let mut run_functions = Vec::new();
    for entry in fs::read_dir(command_dir).expect("Failed to read command directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();
        if path.is_file() && path.extension().unwrap_or_default() == "rs" && path.file_name() != Some("mod.rs".as_ref()) {
            let content = fs::read_to_string(&path).expect("Failed to read file");
            if let Some((command, run_function)) = parse_command_and_run_from_file(&content, path.file_stem().unwrap().to_str().unwrap()) {
                subcommands.push(command);
                run_functions.push(run_function);
            }
        }
    }
    let subcommands_code = subcommands.into_iter().map(|cmd| {
        quote! {
            .subcommand(#cmd)
        }
    });
    let match_arms = run_functions.into_iter().map(|(cmd_name, run_function_call)| {
        quote! {
            Some((#cmd_name, sub_m)) => {
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
            match matches.subcommand() {
                #(#match_arms,)*
                Some((_,_)) => {}
                None => { log::warn!("Please use --help show help message.") }
            }
        }
    };

    TokenStream::from(expanded)
}


fn parse_command_and_run_from_file(content: &str, path: &str) -> Option<(proc_macro2::TokenStream, (proc_macro2::TokenStream, proc_macro2::TokenStream))> {
    let router_line = content.lines().find(|line| line.starts_with("//! router:"))?;
    let description_line = content.lines().find(|line| line.starts_with("//! description:"))?;
    let command_name = router_line.trim_start_matches("//! router:").trim();
    let description = description_line.trim_start_matches("//! description:").trim();
    let args = content.lines().find(|line| line.starts_with("//! args:"));
    let mut options = Vec::new();
    // handle args.
    if let Some(args) = args {
        let args = args.trim_start_matches("//! args:").trim();
        let args = args.split_whitespace().collect::<Vec<&str>>();
        for arg in args {
            if arg.starts_with('<') && (arg.ends_with('>') || arg.ends_with(')')) {
                let arg = arg.trim_start_matches('<').trim_end_matches('>');
                
                //parse arg help.
                //<abc:default_value>(help)
                //parse default_value and arg.
                let arg_help = arg.split('(').nth(1).unwrap_or("").trim_end_matches(')');
                let arg = arg.split('(').collect::<Vec<&str>>()[0].trim_start_matches('<').trim_end_matches('>');
                let arg = arg.split(':').collect::<Vec<&str>>();
                if arg.len() == 1 {
                    let arg_name = proc_macro2::Literal::string(arg[0]);
                    let arg_help_literal = proc_macro2::Literal::string(arg_help);
                    options.push(quote! {
                        .arg(clap::Arg::new(#arg_name).required(true).help(#arg_help_literal))
                    });
                    continue;
                } else if arg.len() == 2 {
                    let default_value = arg[1];
                    let arg_name = proc_macro2::Literal::string(arg[0]);
                    let arg_help_literal = proc_macro2::Literal::string(arg_help);
                    let default_value = proc_macro2::Literal::string(default_value);
                    options.push(quote! {
                        .arg(clap::Arg::new(#arg_name).required(true).default_value(#default_value).help(#arg_help_literal))
                    });
                    continue;
                }
            } else if arg.starts_with('[') && (arg.ends_with(']') || arg.ends_with(')')) {
                let arg_help = arg.split('(').nth(1).unwrap_or("").trim_end_matches(')');
                let arg = arg.split('(').collect::<Vec<&str>>()[0].trim_start_matches('[').trim_end_matches(']');
                let arg = arg.split(':').collect::<Vec<&str>>();
                if arg.len() == 1 {
                    let arg_name = proc_macro2::Literal::string(arg[0]);
                    let arg_help_literal = proc_macro2::Literal::string(arg_help);
                    options.push(quote! {
                        .arg(clap::Arg::new(#arg_name).required(false).help(#arg_help_literal))
                    });
                    continue;
                } else if arg.len() == 2 {
                    let default_value = arg[1];
                    let arg_name = proc_macro2::Literal::string(arg[0]);
                    let arg_help_literal = proc_macro2::Literal::string(arg_help);
                    let default_value = proc_macro2::Literal::string(default_value);
                    options.push(quote! {
                        .arg(clap::Arg::new(#arg_name).required(false).default_value(#default_value).help(#arg_help_literal))
                    });
                    continue;
                }
                // options.push(quote! {
                //     .arg(clap::Arg::new(#arg).required(false))
                // });
            }
        }
    }
    let mut run_str = "".to_string();
    let mut is_run_line = false;
    for line in content.lines() {
        if line.starts_with("//! --") {
            let option_def = line.trim_start_matches("//! ").trim();
            let option_def = option_def.replace("\\,", "PLACEHOLDER");
            let parts: Vec<&str> = option_def.split(',').collect();
            if parts.len() == 2 {
                let option_args = parts[0].split_whitespace().collect::<Vec<&str>>();
                let help_message = parts[1].trim().replace("PLACEHOLDER", ",");
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
        if line.contains("pub fn run(") {
            is_run_line = true;
        }
        if is_run_line {
            run_str.push_str(line);
        }
        if line.contains(")") {
            is_run_line = false;
        }
    }
    // run_fn only save illegal char
    let run_str = run_str.chars().filter(|c| c.is_alphabetic() || "<>(){}^_',:0123456789".contains(*c)).collect::<String>();
    // find run fun
    let run_fn_signature = run_str
        .trim()
        .trim_start_matches("pubfnrun(");
    // then run find first ) and first }
    let run_fn_signature = run_fn_signature.split(')').next().unwrap(); 
    let run_fn_args: Vec<&str> = run_fn_signature.split(',').collect();
    let run_fn_call = run_fn_args.iter().map(|arg| {
        let arg_name = arg.split(':').next().unwrap().trim();
        let type_d = arg.split(':').nth(1).unwrap().trim();
        if type_d.starts_with("Option<") {
            let type_d_inner = type_d.trim_start_matches("Option<");
            let type_d_inner = type_d_inner[..type_d_inner.len()-1].to_string();
            let type_d_ident = syn::parse_str::<syn::Type>(type_d_inner.as_str()).unwrap();
            quote! { { let _res = sub_m.get_one::<#type_d_ident>(#arg_name); if _res == None {
                None
            } else {
                Some(_res.unwrap().clone())
            } } }
        } else {
            let type_d_ident = syn::Ident::new(type_d, proc_macro2::Span::call_site());
            quote! { sub_m.get_one::<#type_d_ident>(#arg_name).unwrap().clone() }   
        }  

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
