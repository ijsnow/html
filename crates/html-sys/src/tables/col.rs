/// The HTML `<col>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/col)
#[doc(alias = "col")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableColumn {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Number of columns spanned by the element
    pub span: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for TableColumn {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<col")?;
        if let Some(field) = self.span.as_ref() {
            write!(writer, r#" span="{field}""#)?;
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
impl crate::ElementDescription for TableColumn {
    fn set_attributes(
        &self,
        attrs: &mut std::collections::HashMap<
            std::borrow::Cow<'static, str>,
            std::borrow::Cow<'static, str>,
        >,
    ) {
        self.global_attrs.add(attrs);
        if let Some(field) = &self.span {
            attrs.insert(std::borrow::Cow::Borrowed("span"), field.to_owned());
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
impl std::fmt::Display for TableColumn {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for TableColumn {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for TableColumn {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
