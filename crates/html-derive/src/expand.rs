use quote::ToTokens;
use syn::{spanned::Spanned, Expr, ExprPath, LitStr, PathSegment, Data, Fields};

use ::{
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        parenthesized,
        parse::{Parse, ParseStream},
        punctuated::Punctuated,
        token::Paren,
        Attribute, DeriveInput, Error, ExprAssign, Path, Token,
    },
};

pub fn expand_derive_to_html_element(input: &DeriveInput) -> Result<TokenStream, Error> {
    let ident = &input.ident;

    let element_description =
        find_element_description(&input.attrs)?.unwrap_or(ElementDescription {
            tag: syn::parse2(quote! { html::text_content::Division })?,
            attributes: Punctuated::new(),
        });

    let target_element = element_description.tag;

    let data_path: PathSegment = syn::parse2(quote! { data })?;

    let mut attributes = vec![];

    for attribute in element_description.attributes {
        let value = &attribute.right;

        let attribute = match attribute.left.as_ref() {
            Expr::Path(ExprPath { path, .. })
                if path.segments.len() == 2 && path.segments.first() == Some(&data_path) =>
            {
                let key = path.segments.last().unwrap();
                let key = LitStr::new(&key.to_token_stream().to_string(), key.span());

                quote! { .data(#key, #value) }
            }
            key @ _ => quote! { .#key(#value) },
        };

        attributes.push(quote! {
            #attribute
        });
    }

    let children = quote! { }; // get_children_element_descriptions_from_data(&input.data)?;

    Ok(quote! {
        impl html_traits::ToHtmlElement for #ident {
            type HtmlElement = #target_element;

            fn to_html_element(&self) -> Self::HtmlElement {
                Self::HtmlElement::builder()
                    #(#attributes)*
                    #children
                    .build()
            }
        }
    })
}

struct ElementDescription {
    tag: Path,
    attributes: Punctuated<ExprAssign, Token![,]>,
}

impl Parse for ElementDescription {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let tag = input.parse()?;

        let attributes = if !input.is_empty() && input.peek(Paren) {
            let content;
            parenthesized!(content in input);
            Punctuated::parse_separated_nonempty(&content)?
        } else {
            Punctuated::new()
        };

        Ok(ElementDescription { tag, attributes })
    }
}

fn find_element_description(attrs: &[Attribute]) -> Result<Option<ElementDescription>, Error> {
    let mut element = None;

    for attr in attrs {
        if attr.path().is_ident("html_element") {
            if element.is_some() {
                return Err(Error::new_spanned(
                    attr,
                    "Only one html_element attribute is accepted.",
                ));
            }

            element = Some(attr.parse_args::<ElementDescription>()?);
        }
    }

    Ok(element)
}

pub fn parse_data(input: &Data) -> Result<TokenStream, Error> {
    match input {
        Data::Enum(data_enum) => todo!(),
        Data::Struct(data_struct) => todo!(),
        Data::Union(data_union) => Err(Error::new(data_union.span(), "Unions are not supported.")),
    }
}

pub fn parse_fields(input: &Fields) -> Result<TokenStream, Error> {
    match input {
        Fields::Unit => todo!(),
        Fields::Named(named) => todo!(),
        Fields::Unnamed(unnamed) => todo!(),
    }
}
