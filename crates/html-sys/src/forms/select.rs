/// The HTML `<select>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/select)
#[doc(alias = "select")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Select {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Hint for form autofill feature
    pub autocomplete: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the form control is disabled
    pub disabled: bool,
    /// Associates the element with a form element
    pub form: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to allow multiple values
    pub multiple: bool,
    /// Name of the element to use for form submission and in the form.elements API
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the control is required for form submission
    pub required: bool,
    /// Size of the control
    pub size: std::option::Option<i64>,
}
impl crate::RenderElement for Select {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<select")?;
        if let Some(field) = self.autocomplete.as_ref() {
            write!(writer, r#" autocomplete="{field}""#)?;
        }
        if self.disabled {
            write!(writer, r#" disabled"#)?;
        }
        if let Some(field) = self.form.as_ref() {
            write!(writer, r#" form="{field}""#)?;
        }
        if self.multiple {
            write!(writer, r#" multiple"#)?;
        }
        if let Some(field) = self.name.as_ref() {
            write!(writer, r#" name="{field}""#)?;
        }
        if self.required {
            write!(writer, r#" required"#)?;
        }
        if let Some(field) = self.size.as_ref() {
            write!(writer, r#" size="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</select>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(select)
    }
    #[cfg(feature = "web-sys")]
    fn apply_attributes(
        &self,
        target: &web_sys::Element,
    ) -> Result<(), wasm_bindgen::JsValue> {
        if let Some(field) = self.autocomplete.as_ref() {
            element.set_attribute("autocomplete", field)?;
        }
        if self.disabled {
            element.set_attribute("disabled", "true")?;
        }
        if let Some(field) = self.form.as_ref() {
            element.set_attribute("form", field)?;
        }
        if self.multiple {
            element.set_attribute("multiple", "true")?;
        }
        if let Some(field) = self.name.as_ref() {
            element.set_attribute("name", field)?;
        }
        if self.required {
            element.set_attribute("required", "true")?;
        }
        if let Some(field) = self.size.as_ref() {
            element.set_attribute("size", field)?;
        }
        self.global_attrs.apply(target)?;
        Ok(())
    }
}
impl std::fmt::Display for Select {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Select {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Select {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
