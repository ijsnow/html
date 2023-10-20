use html::HtmlElement;

pub trait ToHtmlElement {
    type Element: HtmlElement;

    fn to_html_element(&self) -> Self::Element;
}

#[test]
fn test_basic() {
    #[derive(html_derive::ToHtmlElement)]
    #[element(form(class = "hello"))]
    struct MyForm {
        #[element(input(value = "{}", type = "text"))]
        email: String,
        #[element(input(value = "{}", type = "password"))]
        password: String,
    }

    let want = html::forms::Form::builder()
        .class("hello")
        .push(html::forms::Input::builder().value("a@b.com").type_("text"))
        .push(
            html::forms::Input::builder()
                .value("aaaaah")
                .type_("password"),
        );

    let got = MyStruct {
        email: "a@b.com",
        password: "aaaaah",
    }
    .to_html_element();

    assert_eq!(got, want);
}
