/// The HTML `<fieldset>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/fieldset)
#[doc(alias = "fieldset")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fieldset {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Whether the descendant form controls, except any inside legend, are disabled
    pub disabled: bool,
    /// Associates the element with a form element
    pub form: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of the element to use in the form.elements API.
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Fieldset {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<fieldset")?;
        if self.disabled {
            write!(writer, r#" disabled"#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#" form="{field}""#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</fieldset>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(fieldset)
    }
    #[cfg(feature = "web-sys")]
    fn apply_attributes(
        &self,
        target: &web_sys::Element,
    ) -> Result<(), wasm_bindgen::JsValue> {
        if self.disabled {
            element.set_attribute("disabled", "true")?;
        }
        if let Some(field) = self.form.as_ref() {
            element.set_attribute("form", field)?;
        }
        if let Some(field) = self.name.as_ref() {
            element.set_attribute("name", field)?;
        }
        self.global_attrs.apply(target)?;
        Ok(())
    }
}
impl std::fmt::Display for Fieldset {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Fieldset {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Fieldset {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
