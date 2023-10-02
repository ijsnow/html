/// The HTML `<dt>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/dt)
#[doc(alias = "dt")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DescriptionTerm {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
}
impl crate::RenderElement for DescriptionTerm {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<dt")?;
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</dt>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(dt)
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
impl std::fmt::Display for DescriptionTerm {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for DescriptionTerm {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for DescriptionTerm {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
