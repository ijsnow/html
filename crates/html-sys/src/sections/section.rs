/// The HTML `<section>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/section)
#[doc(alias = "section")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Section {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
}
impl crate::RenderElement for Section {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<section")?;
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</section>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(section)
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
impl std::fmt::Display for Section {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Section {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Section {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
