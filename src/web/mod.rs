use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;
use crate::web::css::CSSBuilder;
use crate::web::html::{Html, HtmlFormat};
use crate::website::Category;

#[macro_use]
pub mod css;

#[macro_use]
pub mod html;

pub mod feed {}

pub mod component;

pub trait Renderable {
    fn render(self: Box<Self>, context: &mut dyn RenderContext, out: &mut dyn Write) -> std::io::Result<()>;
}

pub struct ResourceRender(pub PathBuf);

impl Renderable for ResourceRender {
    fn render(self: Box<Self>, _context: &mut dyn RenderContext, out: &mut dyn Write) -> std::io::Result<()> {
        let mut file = File::open(&self.0)?;
        let mut file_buffer = Vec::new();
        file.read_to_end(&mut file_buffer)?;
        out.write_all(&*file_buffer)
    }
}

pub struct SpecialCaseRender();

impl Renderable for SpecialCaseRender {
    fn render(self: Box<Self>, _context: &mut dyn RenderContext, out: &mut dyn Write) -> std::io::Result<()> {
        writeln!(out, "-- This render is special-cased and should be replaced later --")
    }
}

impl<T: Html> Renderable for T {
    fn render(self: Box<Self>, context: &mut dyn RenderContext, out: &mut dyn Write) -> std::io::Result<()> {
        self.build(context, out, HtmlFormat::Indent(0))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct PageRef<'a>(pub &'a str);

impl<'a> Display for PageRef<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Marker type for hypertext links
#[derive(Debug, Clone)]
pub struct HRef(pub String);

#[derive(Debug, Clone)]
pub enum Link {
    ID(String),
    Custom {
        link_title: String,
        destination: HRef,
    },
}

pub trait RenderContext {
    fn title(&self) -> &str;
    fn title_prefix(&self) -> Option<&str>;
    fn resolve_href(&self, link: &Link, from_page: PageRef) -> HRef;
    fn resolve_link_title(&self, link: &Link) -> String;
    fn resolve_link(&self, link: &Link, from_page: PageRef) -> (String, HRef);
    fn resolve_category(&self, category_id: &str) -> &Category;
    fn current_page(&self) -> PageRef;
    fn stylesheet(&mut self) -> &mut CSSBuilder;
    fn stylesheet_link(&self, for_page: PageRef) -> HRef;
    fn global_scripts(&self, for_page: PageRef) -> Vec<HRef>;
}