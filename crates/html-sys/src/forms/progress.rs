/// The HTML `<progress>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/progress)
#[doc(alias = "progress")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Progress {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Current value of the element
    pub value: std::option::Option<f64>,
    /// Upper bound of range
    pub max: std::option::Option<f64>,
}
impl crate::RenderElement for Progress {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<progress")?;
        if let Some(field) = self.value.as_ref() {
            write!(writer, r#" value="{field}""#)?;
        }
        if let Some(field) = self.max.as_ref() {
            write!(writer, r#" max="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</progress>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(progress)
    }
    #[cfg(feature = "web-sys")]
    fn apply_attributes(
        &self,
        target: &web_sys::Element,
    ) -> Result<(), wasm_bindgen::JsValue> {
        if let Some(field) = self.value.as_ref() {
            element.set_attribute("value", field)?;
        }
        if let Some(field) = self.max.as_ref() {
            element.set_attribute("max", field)?;
        }
        self.global_attrs.apply(target)?;
        Ok(())
    }
}
impl std::fmt::Display for Progress {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Progress {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Progress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
