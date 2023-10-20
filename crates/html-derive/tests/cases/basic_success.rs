#![recursion_limit = "1024"]

use html_derive::ToHtmlElement;
use html_traits::ToHtmlElement as _;

#[derive(ToHtmlElement)]
pub struct DefaultToDiv;

fn main() {
    let _div: html::text_content::Division = DefaultToDiv.to_html_element();
}
