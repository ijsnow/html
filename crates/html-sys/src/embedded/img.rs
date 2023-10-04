/// The HTML `<img>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/img)
#[doc(alias = "img")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Image {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Replacement text for use when images are not available
    pub alt: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Images to use in different situations, e.g., high-resolution displays, small monitors, etc.
    pub srcset: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Image sizes for different page layouts
    pub sizes: std::option::Option<std::borrow::Cow<'static, str>>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Name of image map to use
    pub usemap: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether the image is a server-side image map
    pub ismap: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Horizontal dimension
    pub width: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Vertical dimension
    pub height: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Referrer policy for fetches initiated by the element
    pub referrerpolicy: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Decoding hint to use when processing this image for presentation
    pub decoding: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Used when determining loading deferral
    pub loading: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Sets the priority for fetches initiated by the element
    pub fetchpriority: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Image {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<img")?;
        if let Some(field) = self.alt.as_ref() {
            write!(writer, r#" alt="{field}""#)?;
        }
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.srcset.as_ref() {
            write!(writer, r#" srcset="{field}""#)?;
        }
        if let Some(field) = self.sizes.as_ref() {
            write!(writer, r#" sizes="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.usemap.as_ref() {
            write!(writer, r#" usemap="{field}""#)?;
        }
        if let Some(field) = self.ismap.as_ref() {
            write!(writer, r#" ismap="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#" width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#" height="{field}""#)?;
        }
        if let Some(field) = self.referrerpolicy.as_ref() {
            write!(writer, r#" referrerpolicy="{field}""#)?;
        }
        if let Some(field) = self.decoding.as_ref() {
            write!(writer, r#" decoding="{field}""#)?;
        }
        if let Some(field) = self.loading.as_ref() {
            write!(writer, r#" loading="{field}""#)?;
        }
        if let Some(field) = self.fetchpriority.as_ref() {
            write!(writer, r#" fetchpriority="{field}""#)?;
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
impl std::fmt::Display for Image {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Image {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Image {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
