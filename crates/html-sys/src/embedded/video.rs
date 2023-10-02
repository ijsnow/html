/// The HTML `<video>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/video)
#[doc(alias = "video")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Video {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Poster frame to show prior to video playback
    pub poster: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hints how much buffering the media resource will likely need
    pub preload: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hint that the media resource can be started automatically when the page is loaded
    pub autoplay: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Encourage the user agent to display video content within the element's playback area
    pub plays_inline: bool,
    /// Whether to loop the media resource
    pub loop_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to mute the media resource by default
    pub muted: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Show user agent controls
    pub controls: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Horizontal dimension
    pub width: std::option::Option<i64>,
    /// Vertical dimension
    pub height: std::option::Option<i64>,
}
impl crate::RenderElement for Video {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<video")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.poster.as_ref() {
            write!(writer, r#" poster="{field}""#)?;
        }
        if let Some(field) = self.preload.as_ref() {
            write!(writer, r#" preload="{field}""#)?;
        }
        if let Some(field) = self.autoplay.as_ref() {
            write!(writer, r#" autoplay="{field}""#)?;
        }
        if self.plays_inline {
            write!(writer, r#" playsinline"#)?;
        }
        if let Some(field) = self.loop_.as_ref() {
            write!(writer, r#" loop="{field}""#)?;
        }
        if let Some(field) = self.muted.as_ref() {
            write!(writer, r#" muted="{field}""#)?;
        }
        if let Some(field) = self.controls.as_ref() {
            write!(writer, r#" controls="{field}""#)?;
        }
        if let Some(field) = self.width.as_ref() {
            write!(writer, r#" width="{field}""#)?;
        }
        if let Some(field) = self.height.as_ref() {
            write!(writer, r#" height="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</video>")?;
        Ok(())
    }
    #[cfg(feature = "web-sys")]
    fn create_element(&self) -> Result<web_sys::Element, wasm_bindgen::JsValue> {
        gloo::utils::document().create_element(video)
    }
    #[cfg(feature = "web-sys")]
    fn apply_attributes(
        &self,
        target: &web_sys::Element,
    ) -> Result<(), wasm_bindgen::JsValue> {
        if let Some(field) = self.src.as_ref() {
            element.set_attribute("src", field)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            element.set_attribute("crossorigin", field)?;
        }
        if let Some(field) = self.poster.as_ref() {
            element.set_attribute("poster", field)?;
        }
        if let Some(field) = self.preload.as_ref() {
            element.set_attribute("preload", field)?;
        }
        if let Some(field) = self.autoplay.as_ref() {
            element.set_attribute("autoplay", field)?;
        }
        if self.plays_inline {
            element.set_attribute("playsinline", "true")?;
        }
        if let Some(field) = self.loop_.as_ref() {
            element.set_attribute("loop", field)?;
        }
        if let Some(field) = self.muted.as_ref() {
            element.set_attribute("muted", field)?;
        }
        if let Some(field) = self.controls.as_ref() {
            element.set_attribute("controls", field)?;
        }
        if let Some(field) = self.width.as_ref() {
            element.set_attribute("width", field)?;
        }
        if let Some(field) = self.height.as_ref() {
            element.set_attribute("height", field)?;
        }
        self.global_attrs.apply(target)?;
        Ok(())
    }
}
impl std::fmt::Display for Video {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Video {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Video {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
