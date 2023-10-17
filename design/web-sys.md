> Hi there, thanks for asking! Im currently not working on this, no. It'll require some design work, since it's essentially a duplicate backend to html-sys.
>
> A good place to start would be to update the scraper to more accurately parse the WebIDL files for methods, and add them to the JSON.
>
> Another good step is to come up with a plan to support both backends in the same repo. This is where I think the design work would come in, but it's also what would enable us to merge the feature.

By a duplicate back end, do you just mean an alternative to `String` to fold elements into (for the purpose of this issue, `web_sys::Element`)? If so, suppose the additional methods you're talking about are the event handler attributes. Is that correct? If not, what is your vision for `web-sys` support? If so, what if we started by abstracting the requirements for the current `Render` trait?

Thinking from the perspective of someone who might want to implement a renderer with this library (but doesn't know too much about actually implementing a renderer), I think the parts I'd need are the tag of the element, the attributes and the children.

Basic support might be as simple as modifying the [`HtmlElement` trait](https://docs.rs/html/latest/html/trait.HtmlElement.html) to something like the following:

```rust
trait HtmlElement {
    fn tag() -> &'static str;
    fn attributes(&self) -> Vec<(&'static str, AttributeValue)>;
    fn children<'a>(&'a self) -> Vec<DomNode<'a>>;
}

type AttributeValue = Cow<'static, str>;

enum DomNode<'a> {
    Element(&'a dyn HtmlElement),
    Text(&'a str)
}
```

`AttributeValue` could also be the generated enum from [this issue.](https://github.com/yoshuawuyts/html/issues/65)

This trait definition allows flexibility in the type definitions to easily develop abstract renderers while using the builders will allow the framework on top of the renderer to be type safe. I believe this also reflects the state of engines anyway. You can give any html element any value attribute and the browser will do just fine, it just might not have any effect on the dom. Same for children. For most elements, if a child is not valid, the browser will still render the page in quirks mode but it will not fail.

If we wanted to ensure the type safety provided by this project all the way to the boundary of a target type for renderers, we could generate an enum for all possible attributes of a given element and provide a trait to apply that enum to the target type:

```rust
trait Attribute<Target> {
    fn apply(&self, target: Target) -> Result<(), JsValue>;
}
```

I'm sure I'm missing some things but I believe this could support most renderers. Renderers could define their own way of folding elements into their target type as well as gather information for proper diffing of the elements.
