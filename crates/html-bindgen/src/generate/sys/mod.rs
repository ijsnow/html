use super::{CodeFile, Module};
use std::fmt::Write;
use std::{collections::HashMap, iter};

use crate::merge::MergedElement;
use crate::parse::{Attribute, AttributeType};
use crate::{utils, Result};
use indoc::{formatdoc, writedoc};

const INCLUDES: &str = r##"
/// Render an element to a writer.
pub trait RenderElement {
    /// Write the opening tag to a writer.
    fn write_opening_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;

    /// Write the closing tag to a writer, if one is available.
    fn write_closing_tag<W: std::fmt::Write >(&self, writer: &mut W) -> std::fmt::Result;
}

/// Get information about an element.
pub trait ElementDescription {
    /// Access the attributes that have a value for an element.
    fn attributes(&self) -> std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>;

    /// Access the data attributes that have a value for an element.
    fn data(&self) -> std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>;
}

/// Container for `data-*` attributes.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct DataMap {
    map: std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>,
}

impl std::ops::Deref for DataMap {
    type Target = std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl std::ops::DerefMut for DataMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl std::fmt::Display for DataMap {
    fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in self.map.iter() {
            write!(writer, r#" data-{key}="{value}""#)?;
        }
        Ok(())
    }
}
"##;

pub fn generate(
    merged: impl Iterator<Item = Result<MergedElement>>,
    global_attributes: &[Attribute],
    modules: &[Module],
) -> Result<Vec<CodeFile>> {
    let mut output = vec![];
    let mut generated: HashMap<String, Vec<String>> = HashMap::new();

    // generate individual `{element}.rs` files
    for el in merged {
        let el = el?;
        let entry = generated.entry(el.submodule_name.clone());
        entry.or_default().push(el.tag_name.clone());
        let cf = generate_element(el)?;
        output.push(cf);
    }

    // generate `mod.rs` files
    let mut dirs = vec![];
    for (dir, filenames) in generated {
        dirs.push(dir.clone());
        let code = filenames
            .into_iter()
            .map(|name| format!("mod {name};\npub use {name}::*;"))
            .collect::<String>();

        let module = modules.iter().find(|el| &el.name == &dir).unwrap();
        let description = &module.description;
        let code = format!(
            "//! {description}
            {code}"
        );

        output.push(CodeFile {
            filename: "mod.rs".to_owned(),
            code: utils::fmt(&code).expect("could not parse code"),
            dir,
        })
    }
    dirs.sort();

    // generate `lib.rs` file
    let code = dirs
        .into_iter()
        .map(|d| format!("pub mod {d};\n"))
        .chain(iter::once(INCLUDES.to_owned()))
        .chain(iter::once({
            let fields = generate_fields(global_attributes);

            let mut display_attrs = String::new();
            let mut add_attrs = String::new();
            for attr in global_attributes {
                display_attrs.push_str(&generate_attribute_display(&attr));
                add_attrs.push_str(&generate_attribute_adder(&attr));
            }
            formatdoc!(
                r#"

                    /// The "global attributes" struct
                    #[derive(Debug, Clone, PartialEq, Default)]
                    pub struct GlobalAttributes {{
                        {fields}
                    }}

                    impl std::fmt::Display for GlobalAttributes {{
                        fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                            {display_attrs}
                            Ok(())
                        }}
                    }}

                    impl GlobalAttributes {{
                        fn add(&self, attrs: &mut std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>>) {{
                            {add_attrs}
                        }}
                    }}
                    "#
            )
        }))
        .collect::<String>();
    output.push(CodeFile {
        filename: "lib.rs".to_owned(),
        code: utils::fmt(&code)?,
        dir: String::new(),
    });

    Ok(output)
}

/// Generate a single element.
fn generate_element(el: MergedElement) -> Result<CodeFile> {
    let dir = el.submodule_name.clone();
    let MergedElement {
        tag_name,
        struct_name,
        has_closing_tag,
        attributes,
        mdn_link,
        has_global_attributes,
        ..
    } = el;

    let filename = format!("{}.rs", tag_name);
    let fields = generate_fields(&attributes);
    let opening_tag_content = generate_opening_tag(&attributes, &tag_name, has_global_attributes);
    let closing_tag_content = generate_closing_tag(&tag_name, has_closing_tag);

    let mut add_attrs = String::new();
    for attr in attributes {
        add_attrs.push_str(&generate_attribute_adder(&attr));
    }

    let global_field = match has_global_attributes {
        true => format!("global_attrs: crate::GlobalAttributes,"),
        false => String::new(),
    };

    let mut code = formatdoc!(
        r#"/// The HTML `<{tag_name}>` element
        ///
        /// [MDN Documentation]({mdn_link})
        #[doc(alias = "{tag_name}")]
        #[non_exhaustive]
        #[derive(Debug, Clone, PartialEq, Default)]
        pub struct {struct_name} {{
            pub data_map: crate::DataMap,
            {global_field}
            {fields}
        }}

        impl crate::RenderElement for {struct_name} {{
            fn write_opening_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {opening_tag_content}
                Ok(())
            }}

            #[allow(unused_variables)]
            fn write_closing_tag<W: std::fmt::Write>(&self, writer: &mut W) -> std::fmt::Result {{
                {closing_tag_content}
                Ok(())
            }}
        }}

        impl crate::ElementDescription for {struct_name} {{
            fn attributes(&self) -> std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>> {{
                let mut attrs = std::collections::HashMap::new();
                self.global_attrs.add(&mut attrs);
                {add_attrs}
                attrs
            }}

            fn data(&self) -> std::collections::HashMap<std::borrow::Cow<'static, str>, std::borrow::Cow<'static, str>> {{
                (&*self.data_map).clone()
            }}
        }}

        impl std::fmt::Display for {struct_name} {{
            fn fmt(&self, writer: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {{
                use crate::RenderElement;
                self.write_opening_tag(writer)?;
                self.write_closing_tag(writer)?;
                Ok(())
            }}
        }}
    "#
    );

    if has_global_attributes {
        code.push_str(&formatdoc!(
            r#"
            impl std::ops::Deref for {struct_name} {{
                type Target = crate::GlobalAttributes;

                fn deref(&self) -> &Self::Target {{
                    &self.global_attrs
                }}
            }}

            impl std::ops::DerefMut for {struct_name} {{
                fn deref_mut(&mut self) -> &mut Self::Target {{
                    &mut self.global_attrs
                }}
            }}"#
        ));
    }

    Ok(CodeFile {
        filename,
        code: utils::fmt(&code)?,
        dir,
    })
}

fn generate_fields(attributes: &[Attribute]) -> String {
    let mut output = String::new();
    for attr in attributes {
        let description = &attr.description;
        let field_name = &attr.field_name;
        let ty = &attr.ty;
        output.push_str(&match ty {
            AttributeType::Bool => format!(
                "/// {description}
                pub {field_name}: bool,
                "
            ),
            AttributeType::String => format!(
                "/// {description}
             pub {field_name}: std::option::Option<std::borrow::Cow<'static, str>>,
            "
            ),
            _ => format!(
                "/// {description}
             pub {field_name}: std::option::Option<{ty}>,
            "
            ),
        });
    }
    output
}

fn generate_opening_tag(
    attributes: &[Attribute],
    tag_name: &str,
    has_global_attrs: bool,
) -> String {
    let preamble = match tag_name {
        "html" => "<!DOCTYPE html>",
        _ => "",
    };
    let mut output = formatdoc!(
        r#"
        write!(writer, "{preamble}<{tag_name}")?;
    "#
    );
    for attr in attributes {
        output.push_str(&generate_attribute_display(&attr));
    }
    if has_global_attrs {
        output.push_str(&format!(r#"write!(writer, "{{}}", self.global_attrs)?;"#));
    }

    output.push_str(&format!(r#"write!(writer, "{{}}", self.data_map)?;"#));
    writedoc!(&mut output, r#"write!(writer, ">")?;"#).unwrap();
    output
}

fn generate_closing_tag(tag_name: &str, has_closing_tag: bool) -> String {
    if has_closing_tag {
        formatdoc!(
            r#"write!(writer, "</{tag_name}>")?;
        "#
        )
    } else {
        String::new()
    }
}

fn generate_attribute_display(attr: &Attribute) -> String {
    let Attribute {
        name,
        field_name,
        ty,
        ..
    } = &attr;
    match ty {
        AttributeType::Bool => format!(
            r##"if self.{field_name} {{
                    write!(writer, r#" {name}"#)?;
            }}"##
        ),
        AttributeType::String | AttributeType::Integer | AttributeType::Float => format!(
            r##"if let Some(field) = self.{field_name}.as_ref() {{
                write!(writer, r#" {name}="{{field}}""#)?;
            }}"##
        ),
        AttributeType::Identifier(_) => todo!(),
        AttributeType::Enumerable(_) => todo!(),
    }
}

fn generate_attribute_adder(attr: &Attribute) -> String {
    let Attribute {
        name,
        field_name,
        ty,
        ..
    } = &attr;
    match ty {
        AttributeType::Bool => format!(
            r##"if self.{field_name} {{
                    attrs.insert(std::borrow::Cow::Borrowed("{name}"), std::borrow::Cow::Borrowed("true"));
            }}"##
        ),
        // TODO: make enum attribute for value kinds.
        AttributeType::String => format!(
            r##"if let Some(field) = &self.{field_name} {{
                attrs.insert(std::borrow::Cow::Borrowed("{name}"), field.to_owned());
            }}"##
        ),
        AttributeType::Integer | AttributeType::Float => format!(
            r##"if let Some(field) = &self.{field_name} {{
                attrs.insert(std::borrow::Cow::Borrowed("{name}"), std::borrow::Cow::Owned(format!(r#" {{field}}"#)));
            }}"##
        ),
        AttributeType::Identifier(_) => todo!(),
        AttributeType::Enumerable(_) => todo!(),
    }
}
