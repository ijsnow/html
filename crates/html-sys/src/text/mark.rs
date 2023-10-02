/// The HTML `<mark>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/mark)
#[doc(alias = "mark")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MarkText {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
}
impl crate::RenderElement for MarkText {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<mark")?;
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</mark>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(mark)
    }
    #[cfg(feature = "web-sys")]
    fn apply_attributes(
        &self,
        target: &web_sys::Element,
    ) -> Result<(), wasm_bindgen::JsValue> {
        self.global_attrs.apply(target)?;
        Ok(())
    }
}
impl std::fmt::Display for MarkText {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for MarkText {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for MarkText {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
