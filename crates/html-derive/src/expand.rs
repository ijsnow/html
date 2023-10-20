use ::{
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Error},
};

use crate::html::serialize_html;

pub fn expand_derive_renderable(input: &DeriveInput) -> Result<TokenStream, Error> {
    Ok(quote! {})
}
