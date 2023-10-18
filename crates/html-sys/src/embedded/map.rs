/// The HTML `<map>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/map)
#[doc(alias = "map")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ImageMap {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Name of image map to reference from the usemap attribute
    pub name: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for ImageMap {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<map")?;
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
        write!(writer, "</map>")?;
        Ok(())
    }
}
impl crate::ElementDescription for ImageMap {
    fn set_attributes(
        &self,
        attrs: &mut std::collections::HashMap<
            std::borrow::Cow<'static, str>,
            std::borrow::Cow<'static, str>,
        >,
    ) {
        self.global_attrs.add(attrs);
        if let Some(field) = &self.name {
            attrs.insert(std::borrow::Cow::Borrowed("name"), field.to_owned());
        }
    }
    fn set_data(
        &self,
        data: &mut std::collections::HashMap<
            std::borrow::Cow<'static, str>,
            std::borrow::Cow<'static, str>,
        >,
    ) {
        data.extend((&*self.data_map).clone());
    }
}
impl std::fmt::Display for ImageMap {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for ImageMap {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for ImageMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
