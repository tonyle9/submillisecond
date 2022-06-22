mod named_param;
mod router;
mod static_dir;

use proc_macro::TokenStream;
use router::Router;
use static_dir::StaticDir;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(NamedParam, attributes(param))]
pub fn named_param(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    match named_param::NamedParam::try_from(input) {
        Ok(named_param) => named_param.expand().into(),
        Err(err) => err.into_compile_error().into(),
    }
}

#[proc_macro]
pub fn router(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as Router);
    input.expand().into()
}

#[proc_macro]
pub fn static_dir(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StaticDir);
    input.expand().into()
}