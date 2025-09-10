use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, ItemFn, Path};

#[proc_macro_attribute]
pub fn require_access(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let args: Punctuated<Path, Comma> = parse_macro_input!(attr with Punctuated::parse_terminated);

    let sig = &input.sig;
    let vis = &input.vis;
    let block = &input.block;

    let groups: Vec<TokenStream2> = args.iter().map(|path| quote! { #path }).collect();

    let expanded = quote! {
        #vis #sig {
            let user_groups = user.access_groups;
            let authorized = [#(#groups),*].iter().any(|g| user_groups.contains(g));

            if !authorized {
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(ValidationErrorResponse {
                        message: "Acesso negado".to_string(),
                        errors: serde_json::json!(["Usuário não tem permissão"]),
                    }),
                ));
            }

            #block
        }
    };

    TokenStream::from(expanded)
}
