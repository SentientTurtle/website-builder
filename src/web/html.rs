use std::borrow::Cow;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::io::Write;
use crate::web::css::{CSSCallback};
use crate::web::{HRef, RenderContext};

#[derive(Copy, Clone, Debug)]
pub enum HtmlFormat {
    Indent(usize),
    Preformatted
}

impl HtmlFormat {
    pub fn is_preformat(self) -> bool {
        if let HtmlFormat::Preformatted = self {
            true
        } else {
            false
        }
    }
}

pub trait Html: Debug {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool;

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()>;
    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()>;
}

impl<C1: Html, C2: Html> Html for (C1, C2) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context) && self.1.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}
impl<C1: Html, C2: Html, C3: Html> Html for (C1, C2, C3) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context) && self.1.is_inline(context) && self.2.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
            self.2.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.2.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

impl<C1: Html, C2: Html, C3: Html, C4: Html> Html for (C1, C2, C3, C4) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context)
            && self.1.is_inline(context)
            && self.2.is_inline(context)
            && self.3.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
            self.2.build(context, html_out, format)?;
            self.3.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.2.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.3.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

impl<C1: Html, C2: Html, C3: Html, C4: Html, C5: Html> Html for (C1, C2, C3, C4, C5) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context)
            && self.1.is_inline(context)
            && self.2.is_inline(context)
            && self.3.is_inline(context)
            && self.4.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
            self.2.build(context, html_out, format)?;
            self.3.build(context, html_out, format)?;
            self.4.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.2.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.3.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.4.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

impl<C1: Html, C2: Html, C3: Html, C4: Html, C5: Html, C6: Html> Html for (C1, C2, C3, C4, C5, C6) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context)
            && self.1.is_inline(context)
            && self.2.is_inline(context)
            && self.3.is_inline(context)
            && self.4.is_inline(context)
            && self.5.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
            self.2.build(context, html_out, format)?;
            self.3.build(context, html_out, format)?;
            self.4.build(context, html_out, format)?;
            self.5.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.2.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.3.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.4.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.5.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

impl<C1: Html, C2: Html, C3: Html, C4: Html, C5: Html, C6: Html, C7: Html> Html for (C1, C2, C3, C4, C5, C6, C7) {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.0.is_inline(context)
            && self.1.is_inline(context)
            && self.2.is_inline(context)
            && self.3.is_inline(context)
            && self.4.is_inline(context)
            && self.5.is_inline(context)
            && self.6.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if self.is_inline(context) {
            self.0.build(context, html_out, format)?;
            self.1.build(context, html_out, format)?;
            self.2.build(context, html_out, format)?;
            self.3.build(context, html_out, format)?;
            self.4.build(context, html_out, format)?;
            self.5.build(context, html_out, format)?;
            self.6.build(context, html_out, format)?;
        } else {
            self.0.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.1.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.2.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.3.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.4.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.5.build(context, html_out, format)?;
            if let HtmlFormat::Indent(indent) = format {
                writeln!(html_out)?;
                write!(html_out, "{:indent$}", "", indent = indent * 4)?;
            }
            self.6.build(context, html_out, format)?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

//noinspection DuplicatedCode;  Implementation for both array and vec
impl<C: Html, const N: usize> Html for [C; N] {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.iter().all(|item| item.is_inline(context))
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if let HtmlFormat::Indent(indent) = format && !self.is_inline(context) {
            let mut i = self.len();
            for item in self {
                item.build(context, html_out, format)?;
                i -= 1;
                if i > 0 {
                    writeln!(html_out)?;
                    write!(html_out, "{:indent$}", "", indent = indent * 4)?;
                }
            }
        } else {
            for item in self {
                item.build(context, html_out, format)?;
            }
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

//noinspection DuplicatedCode;  Implementation for both array and vec
impl<const N: usize> Html for [Box<dyn Html>; N] {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.iter().all(|item| item.is_inline(context))
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if let HtmlFormat::Indent(indent) = format && !self.is_inline(context) {
            let mut i = self.len();
            for item in self {
                item.build_boxed(context, html_out, format)?;
                i -= 1;
                if i > 0 {
                    writeln!(html_out)?;
                    write!(html_out, "{:indent$}", "", indent = indent * 4)?;
                }
            }
        } else {
            for item in self {
                item.build_boxed(context, html_out, format)?;
            }
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

//noinspection DuplicatedCode;  Implementation for both array and vec
impl<H: Html> Html for Vec<H> {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.iter().all(|item| item.is_inline(context))
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if let HtmlFormat::Indent(indent) = format && !self.is_inline(context) {
            let mut i = self.len();
            for item in self {
                item.build(context, html_out, format)?;
                i -= 1;
                if i > 0 {
                    writeln!(html_out)?;
                    write!(html_out, "{:indent$}", "", indent = indent * 4)?;
                }
            }
        } else {
            for item in self {
                item.build(context, html_out, format)?;
            }
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

//noinspection DuplicatedCode;  Implementation for both array and vec
impl Html for Vec<Box<dyn Html>> {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.iter().all(|item| item.is_inline(context))
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        if let HtmlFormat::Indent(indent) = format && !self.is_inline(context) {
            let mut i = self.len();
            for item in self {
                item.build_boxed(context, html_out, format)?;
                i -= 1;
                if i > 0 {
                    writeln!(html_out)?;
                    write!(html_out, "{:indent$}", "", indent = indent * 4)?;
                }
            }
        } else {
            for item in self {
                item.build_boxed(context, html_out, format)?;
            }
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct Tag {
    name: &'static str,
    is_void: bool,
}

impl Tag {
    pub fn from_name(name: &'static str) -> Tag {
        let name_lower = name.to_ascii_lowercase();
        match &*name_lower {
            "!doctype" => Tag { name, is_void: true },
            "area" => Tag { name, is_void: true },
            "base" => Tag { name, is_void: true },
            "br" => Tag { name, is_void: true },
            "col" => Tag { name, is_void: true },
            "embed" => Tag { name, is_void: true },
            "hr" => Tag { name, is_void: true },
            "img" => Tag { name, is_void: true },
            "input" => Tag { name, is_void: true },
            "link" => Tag { name, is_void: true },
            "meta" => Tag { name, is_void: true },
            "source" => Tag { name, is_void: true },
            "track" => Tag { name, is_void: true },
            "wbr" => Tag { name, is_void: true },
            _ => Tag { name, is_void: false },
        }
    }
}

pub trait AttributeValue {
    fn into_optional_string(self) -> Option<String>;
}

impl AttributeValue for () {
    fn into_optional_string(self) -> Option<String> {
        None
    }
}

impl AttributeValue for String {
    fn into_optional_string(self) -> Option<String> {
        Some(self.into())
    }
}

impl AttributeValue for &str {
    fn into_optional_string(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl AttributeValue for Option<String> {
    fn into_optional_string(self) -> Option<String> {
        self
    }
}

impl AttributeValue for Option<&str> {
    fn into_optional_string(self) -> Option<String> {
        self.map(str::to_string)
    }
}

impl AttributeValue for HRef {
    fn into_optional_string(self) -> Option<String> {
        Some(self.0)
    }
}

#[derive(Debug)]
pub struct HtmlElement {
    tag: Tag,
    attributes: HashMap<&'static str, Option<String>>,
    content: Vec<Box<dyn Html>>,
    enable_inline: bool,
    preformatted_content: bool
}

impl HtmlElement {
    pub fn new(tag: Tag) -> Self {
        Self { tag, attributes: HashMap::new(), content: Vec::new(), enable_inline: false, preformatted_content: false }
    }

    pub fn attribute<S: AttributeValue>(mut self, attribute_name: &'static str, value: S) -> Self {
        let old_value = self.attributes.insert(attribute_name, value.into_optional_string());
        debug_assert!(old_value.is_none(), "attempt to override attribute {}", attribute_name);
        self
    }
    pub fn attribute_opt<S: AttributeValue>(mut self, attribute_name: &'static str, value_opt: Option<S>) -> Self {
        if let Some(value) = value_opt {
            let old_value = self.attributes.insert(attribute_name, value.into_optional_string());
            debug_assert!(old_value.is_none(), "attempt to override attribute {}", attribute_name);
        }
        self
    }

    pub fn content<C: Html + 'static>(mut self, content: C) -> Self {
        if self.tag.is_void {
            panic!("Attempt to set content for void element {}", self.tag.name);
        }
        self.content.push(Box::new(content));
        self
    }

    pub fn content_opt<C: Html + 'static>(mut self, content_opt: Option<C>) -> Self {
        if self.tag.is_void {
            panic!("Attempt to set content for void element {}", self.tag.name);
        }
        if let Some(content) = content_opt {
            self.content.push(Box::new(content));
        }
        self
    }

    pub fn inline(mut self, can_inline: bool) -> Self {
        self.enable_inline = can_inline;
        self
    }

    pub fn preformatted_content(mut self, preformatted: bool) -> Self {
        self.preformatted_content = preformatted;
        self
    }
}

impl Html for HtmlElement {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.enable_inline && self.content.iter().all(|c| c.is_inline(context))
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        const ATTRIBUTE_ORDER: [&str; 11] = ["id", "name", "class", "src", "for", "type", "href", "value", "title", "alt", "role"];
        let mut attributes = self.attributes.into_iter().collect::<Vec<_>>();
        attributes.sort_unstable_by(|(r_key, _), (l_key, _)| {
            match (
                ATTRIBUTE_ORDER.iter().position(|attribute| attribute == r_key),
                ATTRIBUTE_ORDER.iter().position(|attribute| attribute == l_key)
            ) {
                (Some(r_idx), Some(l_idx)) => r_idx.cmp(&l_idx).reverse(),
                (Some(_), None) => Ordering::Greater,
                (None, Some(_)) => Ordering::Less,
                (None, None) => r_key.cmp(l_key)
            }
        });

        if !self.tag.is_void {
            write!(html_out, "<{}", self.tag.name)?;

            for (attribute_name, attribute_value) in attributes {
                if let Some(value) = attribute_value && !value.is_empty() {
                    write!(html_out, " {}=\"{}\"", attribute_name, value.replace('\"', "&quot;"))?
                } else {
                    write!(html_out, " {}", attribute_name)?
                }
            }

            if self.content.len() == 0 {
                write!(html_out, "></{}>", self.tag.name)?;
            } else {
                let will_inline = self.enable_inline && self.content.iter().all(|item| item.is_inline(context));
                if let HtmlFormat::Indent(indent) = format && !will_inline && !self.preformatted_content {
                    writeln!(html_out, ">")?;
                    write!(html_out, "{:indent$}", "", indent = (indent + 1) * 4)?;
                    for content in self.content {
                        content.build_boxed(context, html_out, HtmlFormat::Indent(indent + 1))?;
                    }
                    writeln!(html_out)?;
                    write!(html_out, "{:indent$}", "", indent = indent * 4)?;
                    write!(html_out, "</{}>", self.tag.name)?;
                } else {
                    write!(html_out, ">")?;
                    for content in self.content {
                        if self.preformatted_content {
                            content.build_boxed(context, html_out, HtmlFormat::Preformatted)?;
                        } else {
                            content.build_boxed(context, html_out, format)?;
                        }
                    }
                    write!(html_out, "</{}>", self.tag.name)?;
                }
            }
        } else {
            debug_assert!(self.content.len() == 0, "content specified for void-element");

            write!(html_out, "<{}", self.tag.name)?;
            for (attribute_name, attribute_value) in attributes {
                if let Some(value) = attribute_value && !value.is_empty() {
                    write!(html_out, " {}=\"{}\"", attribute_name, value.replace('\"', "&quot;"))?
                } else {
                    write!(html_out, " {}", attribute_name)?
                }
            }
            write!(html_out, ">")?;
        }
        Ok(())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct RawHtml(pub String);

impl Html for RawHtml {
    fn is_inline(&self, _context: &mut dyn RenderContext) -> bool {
        !self.0.contains('\n')
    }

    fn build(self, _context: &mut dyn RenderContext, html_out: &mut dyn Write, _format: HtmlFormat) -> std::io::Result<()> {
        html_out.write_all(self.0.as_bytes())
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct HtmlPlaintext(pub String);

impl Html for HtmlPlaintext {
    fn is_inline(&self, _context: &mut dyn RenderContext) -> bool {
        !self.0.contains('\n')
    }

    fn build(self, _context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        // Replacing these in one pass would be a lot more efficient, but :effort:
        let text = self.0.replace("\r\n", "\n")
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#x27;");

        if let HtmlFormat::Indent(indent) = format {
            text.lines()
                .map(Cow::Borrowed)
                .intersperse(Cow::Owned(format!("\n{:indent$}", "", indent = indent * 4)))
                .try_for_each(|line| html_out.write_all(line.as_ref().as_bytes()))
        } else {
            html_out.write_all(text.as_bytes())
        }
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct Component {
    pub(super) content: HtmlElement,
    pub(super) style: Vec<CSSCallback>,
}

impl Html for Component {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        self.content.is_inline(context)
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.style.iter().for_each(|style| context.stylesheet().register(*style));
        self.content.build(context, html_out, format)
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

macro_rules! component {
    ($classname:ident, [$($style_class:ident),*], $style:expr) => {
        pub fn $classname<C: $crate::web::html::Html + 'static>(content: C) -> $crate::web::html::Component {
            fn style_callback() -> $crate::web::css::CSSRule {
                 ($crate::web::css::CSSQuery::None, concat!(".", stringify!($classname)), Box::new($style))
            }

            $crate::web::html::Component {
                content: element("div")
                    .attribute("class", concat!(stringify!($classname), $(" ", stringify!($style_class)),*))
                    .content(content),
                style: vec![style_callback]
            }
        }
    };
    ($classname:ident, [$($style_class:ident),*], fn($($param_name:ident: $param_type:ty),*) {$mapper:expr}, $style:expr) => {
        pub fn $classname($($param_name: $param_type),*) -> $crate::web::html::Component {
            fn style_callback() -> $crate::web::css::CSSRule {
                ($crate::web::css::CSSQuery::None, concat!(".", stringify!($classname)), Box::new($style))
            }

            $crate::web::html::Component {
                content: element("div")
                    .attribute("class", concat!(stringify!($classname), $(" ", stringify!($style_class)),*))
                    .content($mapper),
                style: vec![style_callback],
            }
        }
    };
}
