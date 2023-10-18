/// The HTML `<audio>` element
///
/// [MDN Documentation](https://developer.mozilla.org/en-US/docs/Web/HTML/Element/audio)
#[doc(alias = "audio")]
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Audio {
    pub data_map: crate::DataMap,
    global_attrs: crate::GlobalAttributes,
    /// Address of the resource
    pub src: std::option::Option<std::borrow::Cow<'static, str>>,
    /// How the element handles crossorigin requests
    pub crossorigin: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hints how much buffering the media resource will likely need
    pub preload: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Hint that the media resource can be started automatically when the page is loaded
    pub autoplay: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to loop the media resource
    pub loop_: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Whether to mute the media resource by default
    pub muted: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Show user agent controls
    pub controls: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Describes the role(s) the current element plays in the context of the document.
    pub role: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the currently active element when DOM focus is on a composite widget, combobox, textbox, group, or application.
    pub aria_active_descendant_element: std::option::Option<
        std::borrow::Cow<'static, str>,
    >,
    /// Indicates whether assistive technologies will present all, or only parts of, the changed region based on the change notifications defined by the aria-relevant attribute.
    pub aria_atomic: bool,
    /// Defines a string value that labels the current element, which is intended to be converted into Braille. See related aria-label.
    pub aria_braille_label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a human-readable, author-localized abbreviated description for the role of an element, which is intended to be converted into Braille. See related aria-roledescription.
    pub aria_braille_role_description: std::option::Option<
        std::borrow::Cow<'static, str>,
    >,
    /// Indicates an element is being modified and that assistive technologies could wait until the modifications are complete before exposing them to the user.
    pub aria_busy: bool,
    /// Identifies the element (or elements) whose contents or presence are controlled by the current element. See related aria-owns.
    pub aria_controls_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates the element that represents the current item within a container or set of related elements.
    pub aria_current: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that describes the object. See related aria-labelledby and aria-description.
    pub aria_described_by_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a string value that describes or annotates the current element. See related aria-describedby.
    pub aria_description: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that provide additional information related to the object. See related aria-describedby.
    pub aria_details_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that the element is perceivable but disabled, so it is not editable or otherwise operable. See related aria-hidden and aria-readonly.
    pub aria_disabled: bool,
    /// [Deprecated in ARIA 1.1] Indicates what functions can be performed when a dragged object is released on the drop target.
    pub aria_drop_effect: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that provides an error message for an object. See related aria-invalid and aria-describedby.
    pub aria_error_message_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether a grouping element owned or controlled by this element is expanded or collapsed.
    pub aria_expanded: bool,
    /// Identifies the next element (or elements) in an alternate reading order of content which, at the user's discretion, allows assistive technology to override the general default of reading in document source order.
    pub aria_flow_to_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// [Deprecated in ARIA 1.1] Indicates an element's "grabbed" state in a drag-and-drop operation.
    pub aria_grabbed: bool,
    /// Indicates the availability and type of interactive popup element, such as menu or dialog, that can be triggered by an element.
    pub aria_has_popup: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates whether the element is exposed to an accessibility API. See related aria-disabled.
    pub aria_hidden: bool,
    /// Indicates the entered value does not conform to the format expected by the application. See related aria-errormessage.
    pub aria_invalid: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines keyboard shortcuts that an author has implemented to activate or give focus to an element.
    pub aria_key_shortcuts: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a string value that labels the current element. See related aria-labelledby.
    pub aria_label: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies the element (or elements) that labels the current element. See related aria-label and aria-describedby.
    pub aria_labelled_by_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates that an element will be updated, and describes the types of updates the user agents, assistive technologies, and user can expect from the live region.
    pub aria_live: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Identifies an element (or elements) in order to define a visual, functional, or contextual parent/child relationship between DOM elements where the DOM hierarchy cannot be used to represent the relationship. See related aria-controls.
    pub aria_owns_elements: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Indicates what notifications the user agent will trigger when the accessibility tree within a live region is modified. See related aria-atomic.
    pub aria_relevant: std::option::Option<std::borrow::Cow<'static, str>>,
    /// Defines a human-readable, author-localized description for the role of an element.
    pub aria_role_description: std::option::Option<std::borrow::Cow<'static, str>>,
}
impl crate::RenderElement for Audio {
    fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "<audio")?;
        if let Some(field) = self.src.as_ref() {
            write!(writer, r#" src="{field}""#)?;
        }
        if let Some(field) = self.crossorigin.as_ref() {
            write!(writer, r#" crossorigin="{field}""#)?;
        }
        if let Some(field) = self.preload.as_ref() {
            write!(writer, r#" preload="{field}""#)?;
        }
        if let Some(field) = self.autoplay.as_ref() {
            write!(writer, r#" autoplay="{field}""#)?;
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
        if let Some(field) = self.role.as_ref() {
            write!(writer, r#" role="{field}""#)?;
        }
        if let Some(field) = self.aria_active_descendant_element.as_ref() {
            write!(writer, r#" aria-activedescendant="{field}""#)?;
        }
        if self.aria_atomic {
            write!(writer, r#" aria-atomic"#)?;
        }
        if let Some(field) = self.aria_braille_label.as_ref() {
            write!(writer, r#" aria-braillelabel="{field}""#)?;
        }
        if let Some(field) = self.aria_braille_role_description.as_ref() {
            write!(writer, r#" aria-brailleroledescription="{field}""#)?;
        }
        if self.aria_busy {
            write!(writer, r#" aria-busy"#)?;
        }
        if let Some(field) = self.aria_controls_elements.as_ref() {
            write!(writer, r#" aria-controls="{field}""#)?;
        }
        if let Some(field) = self.aria_current.as_ref() {
            write!(writer, r#" aria-current="{field}""#)?;
        }
        if let Some(field) = self.aria_described_by_elements.as_ref() {
            write!(writer, r#" aria-describedby="{field}""#)?;
        }
        if let Some(field) = self.aria_description.as_ref() {
            write!(writer, r#" aria-description="{field}""#)?;
        }
        if let Some(field) = self.aria_details_elements.as_ref() {
            write!(writer, r#" aria-details="{field}""#)?;
        }
        if self.aria_disabled {
            write!(writer, r#" aria-disabled"#)?;
        }
        if let Some(field) = self.aria_drop_effect.as_ref() {
            write!(writer, r#" aria-dropeffect="{field}""#)?;
        }
        if let Some(field) = self.aria_error_message_elements.as_ref() {
            write!(writer, r#" aria-errormessage="{field}""#)?;
        }
        if self.aria_expanded {
            write!(writer, r#" aria-expanded"#)?;
        }
        if let Some(field) = self.aria_flow_to_elements.as_ref() {
            write!(writer, r#" aria-flowto="{field}""#)?;
        }
        if self.aria_grabbed {
            write!(writer, r#" aria-grabbed"#)?;
        }
        if let Some(field) = self.aria_has_popup.as_ref() {
            write!(writer, r#" aria-haspopup="{field}""#)?;
        }
        if self.aria_hidden {
            write!(writer, r#" aria-hidden"#)?;
        }
        if let Some(field) = self.aria_invalid.as_ref() {
            write!(writer, r#" aria-invalid="{field}""#)?;
        }
        if let Some(field) = self.aria_key_shortcuts.as_ref() {
            write!(writer, r#" aria-keyshortcuts="{field}""#)?;
        }
        if let Some(field) = self.aria_label.as_ref() {
            write!(writer, r#" aria-label="{field}""#)?;
        }
        if let Some(field) = self.aria_labelled_by_elements.as_ref() {
            write!(writer, r#" aria-labelledby="{field}""#)?;
        }
        if let Some(field) = self.aria_live.as_ref() {
            write!(writer, r#" aria-live="{field}""#)?;
        }
        if let Some(field) = self.aria_owns_elements.as_ref() {
            write!(writer, r#" aria-owns="{field}""#)?;
        }
        if let Some(field) = self.aria_relevant.as_ref() {
            write!(writer, r#" aria-relevant="{field}""#)?;
        }
        if let Some(field) = self.aria_role_description.as_ref() {
            write!(writer, r#" aria-roledescription="{field}""#)?;
        }
        write!(writer, "{}", self.global_attrs)?;
        write!(writer, "{}", self.data_map)?;
        write!(writer, ">")?;
        Ok(())
    }
    #[allow(unused_variables)]
    fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {
        write!(writer, "</audio>")?;
        Ok(())
    }
}
impl crate::ElementDescription for Audio {
    fn attributes(
        &self,
    ) -> std::collections::HashMap<
        std::borrow::Cow<'static, str>,
        std::borrow::Cow<'static, str>,
    > {
        let mut attrs = std::collections::HashMap::new();
        self.global_attrs.add(&mut attrs);
        if let Some(field) = &self.src {
            attrs.insert(std::borrow::Cow::Borrowed("src"), field.to_owned());
        }
        if let Some(field) = &self.crossorigin {
            attrs.insert(std::borrow::Cow::Borrowed("crossorigin"), field.to_owned());
        }
        if let Some(field) = &self.preload {
            attrs.insert(std::borrow::Cow::Borrowed("preload"), field.to_owned());
        }
        if let Some(field) = &self.autoplay {
            attrs.insert(std::borrow::Cow::Borrowed("autoplay"), field.to_owned());
        }
        if let Some(field) = &self.loop_ {
            attrs.insert(std::borrow::Cow::Borrowed("loop"), field.to_owned());
        }
        if let Some(field) = &self.muted {
            attrs.insert(std::borrow::Cow::Borrowed("muted"), field.to_owned());
        }
        if let Some(field) = &self.controls {
            attrs.insert(std::borrow::Cow::Borrowed("controls"), field.to_owned());
        }
        if let Some(field) = &self.role {
            attrs.insert(std::borrow::Cow::Borrowed("role"), field.to_owned());
        }
        if let Some(field) = &self.aria_active_descendant_element {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-activedescendant"),
                    field.to_owned(),
                );
        }
        if self.aria_atomic {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-atomic"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_braille_label {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-braillelabel"),
                    field.to_owned(),
                );
        }
        if let Some(field) = &self.aria_braille_role_description {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-brailleroledescription"),
                    field.to_owned(),
                );
        }
        if self.aria_busy {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-busy"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_controls_elements {
            attrs.insert(std::borrow::Cow::Borrowed("aria-controls"), field.to_owned());
        }
        if let Some(field) = &self.aria_current {
            attrs.insert(std::borrow::Cow::Borrowed("aria-current"), field.to_owned());
        }
        if let Some(field) = &self.aria_described_by_elements {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-describedby"),
                    field.to_owned(),
                );
        }
        if let Some(field) = &self.aria_description {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-description"),
                    field.to_owned(),
                );
        }
        if let Some(field) = &self.aria_details_elements {
            attrs.insert(std::borrow::Cow::Borrowed("aria-details"), field.to_owned());
        }
        if self.aria_disabled {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-disabled"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_drop_effect {
            attrs
                .insert(std::borrow::Cow::Borrowed("aria-dropeffect"), field.to_owned());
        }
        if let Some(field) = &self.aria_error_message_elements {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-errormessage"),
                    field.to_owned(),
                );
        }
        if self.aria_expanded {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-expanded"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_flow_to_elements {
            attrs.insert(std::borrow::Cow::Borrowed("aria-flowto"), field.to_owned());
        }
        if self.aria_grabbed {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-grabbed"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_has_popup {
            attrs.insert(std::borrow::Cow::Borrowed("aria-haspopup"), field.to_owned());
        }
        if self.aria_hidden {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-hidden"),
                    std::borrow::Cow::Borrowed("true"),
                );
        }
        if let Some(field) = &self.aria_invalid {
            attrs.insert(std::borrow::Cow::Borrowed("aria-invalid"), field.to_owned());
        }
        if let Some(field) = &self.aria_key_shortcuts {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-keyshortcuts"),
                    field.to_owned(),
                );
        }
        if let Some(field) = &self.aria_label {
            attrs.insert(std::borrow::Cow::Borrowed("aria-label"), field.to_owned());
        }
        if let Some(field) = &self.aria_labelled_by_elements {
            attrs
                .insert(std::borrow::Cow::Borrowed("aria-labelledby"), field.to_owned());
        }
        if let Some(field) = &self.aria_live {
            attrs.insert(std::borrow::Cow::Borrowed("aria-live"), field.to_owned());
        }
        if let Some(field) = &self.aria_owns_elements {
            attrs.insert(std::borrow::Cow::Borrowed("aria-owns"), field.to_owned());
        }
        if let Some(field) = &self.aria_relevant {
            attrs.insert(std::borrow::Cow::Borrowed("aria-relevant"), field.to_owned());
        }
        if let Some(field) = &self.aria_role_description {
            attrs
                .insert(
                    std::borrow::Cow::Borrowed("aria-roledescription"),
                    field.to_owned(),
                );
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
impl std::fmt::Display for Audio {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use crate::RenderElement;
        self.write_opening_tag(writer)?;
        self.write_closing_tag(writer)?;
        Ok(())
    }
}
impl std::ops::Deref for Audio {
    type Target = crate::GlobalAttributes;
    fn deref(&self) -> &Self::Target {
        &self.global_attrs
    }
}
impl std::ops::DerefMut for Audio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.global_attrs
    }
}
