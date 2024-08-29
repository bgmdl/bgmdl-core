extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};
use std::fs;

#[proc_macro]
pub fn pub_handlers(_input: TokenStream) -> TokenStream {
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