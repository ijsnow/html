mod expand;

use ::{
    proc_macro::TokenStream,
    syn::{parse_macro_input, DeriveInput, Error},
};

#[proc_macro_derive(ToHtmlElement)]
pub fn derive_to_html(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    expand::expand_derive_renderable(&input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
