#![recursion_limit = "1024"]

use html_derive::ToHtmlElement;
use html_traits::ToHtmlElement as _;

#[derive(ToHtmlElement)]
#[html_element(html::text_content::Division(class = "hello", data::subject = "stranger"))]
pub struct DefaultToDiv {
    child: &'static str,
}

fn main() {
    let got: html::text_content::Division = DefaultToDiv {
        child: "Hello, stranger.",
    }
    .to_html_element();

    let want = html::text_content::Division::builder()
        .class("hello")
        .data("subject", "stranger")
        .push("Hello, stranger.")
        .build();

    assert_eq!(got, want);
}
