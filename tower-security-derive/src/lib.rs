use darling::FromDeriveInput;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(SessionData, attributes(session))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let opts = SessionOpts::from_derive_input(&input).unwrap();
    let ident = opts.ident;
    let cookie_name = opts.cookie_name;
    quote! {
        impl tower_security::session::SessionData for #ident {
            const COOKIE_NAME: &'static str = #cookie_name;
        }
    }
    .into()
}

#[derive(FromDeriveInput)]
#[darling(attributes(session), forward_attrs(allow, doc, cfg))]
struct SessionOpts {
    ident: syn::Ident,
    cookie_name: String,
}
