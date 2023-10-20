use ::{
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Error},
};

pub fn expand_derive_renderable(input: &DeriveInput) -> Result<TokenStream, Error> {
    let ident = &input.ident;

    Ok(quote! {
        impl html_traits::ToHtmlElement for #ident {
            type HtmlElement = html::text_content::Division;

            fn to_html_element(&self) -> Self::HtmlElement {
                todo!()
            }
        }
    })
}
