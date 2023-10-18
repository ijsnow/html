/// The HTML `<source>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/source)
#[doc(alias = "source")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct MediaSource {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Type of embedded resource
    pub type_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Applicable media
    pub media: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for MediaSource {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<source")?;
        if let Some(field) = self.type_.as_ref() {
            write!(writer, r#" type="{field}""#)?;
        }
        if let Some(field) = self.media.as_ref() {
            write!(writer, r#" media="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        Ok(())
    }
}
impl crate::ElementDescription for MediaSource {
    fn attributes(
        &self,
    ) -> std::collections::HashMap<
        std::borrow::Cow<'static, str>,
        std::borrow::Cow<'static, str>,
    > {
        let mut attrs = std::collections::HashMap::new();
        self.global_attrs.add(&mut attrs);
        if let Some(field) = &self.type_ {
            attrs.insert(std::borrow::Cow::Borrowed("type"), field.to_owned());
        }
        if let Some(field) = &self.media {
            attrs.insert(std::borrow::Cow::Borrowed("media"), field.to_owned());
        }
        attrs
    }
    fn data(
        &self,
    ) -> &std::collections::HashMap<
        std::borrow::Cow<'static, str>,
        std::borrow::Cow<'static, str>,
    > {
        &*self.data_map
    }
}
impl std::fmt::Display for MediaSource {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for MediaSource {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for MediaSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
