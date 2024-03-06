use std::io::Write;
use std::vec;
use chrono::{DateTime, Utc};
use crate::util;
use crate::util::{DisplayExt, Language, VecExt};
use crate::web::html::{Component, Html, HtmlElement, HtmlFormat, HtmlPlaintext, RawHtml, Tag};
use crate::web::{HRef, Link, RenderContext};
use crate::web::css::{CSSQuery, CSSRule};

pub fn html_raw<S: Into<String>>(text: S) -> RawHtml {
    RawHtml(text.into())
}

pub fn html_text<S: Into<String>>(text: S) -> HtmlPlaintext {
    HtmlPlaintext(text.into())
}

fn element(tag_name: &'static str) -> HtmlElement {
    HtmlElement::new(Tag::from_name(tag_name))
}

pub fn html_break() -> HtmlElement {
    element("br").inline(true)
}

pub fn html_horizontal_rule() -> impl Html {
    element("hr").inline(true)
}

pub fn html_span<C: Html + 'static>(content: C) -> HtmlElement {
    element("span")
        .inline(true)
        .content(content)
}

pub fn html_italics<C: Html + 'static>(content: C) -> HtmlElement {
    element("i")
        .inline(true)
        .content(content)
}

pub fn html_bold<C: Html + 'static>(content: C) -> HtmlElement {
    element("b")
        .inline(true)
        .content(content)
}

pub fn html_strong<C: Html + 'static>(content: C) -> HtmlElement {
    element("strong")
        .inline(true)
        .content(content)
}

pub fn html_preformatted<C: Html + 'static>(content: C) -> HtmlElement {
    element("pre")
        .inline(true)
        .preformatted_content(true)
        .content(content)
}

pub fn html_paragraph<C: Html + 'static>(content: C) -> HtmlElement {
    element("p").content(content)
}

pub fn html_code<C: Html + 'static>(content: C) -> HtmlElement {
    element("code")
        .inline(true)
        .content(content)
}

pub fn html_blockquote<C: Html + 'static>(content: C) -> HtmlElement {
    element("blockquote")
        .inline(true)
        .content(content)
}

pub fn html_checkbox(value: bool, enabled: bool) -> HtmlElement {
    element("input")
        .attribute("type", "checkbox")
        .attribute_opt::<Option<&str>>("checked", if value { Some(None) } else { None })
        .attribute_opt::<Option<&str>>("disabled", if enabled { None } else { Some(None) })
        .inline(true)
}


#[derive(Debug)]
pub struct LinkText<C: Html> {
    link: Link,
    title: Option<String>,
    content: Option<C>,
}

pub fn html_link(link: Link, title: Option<String>) -> LinkText<HtmlPlaintext> {
    LinkText {
        link,
        title,
        content: None,
    }
}

pub fn html_link_content<C: Html>(link: Link, title: Option<String>, content: C) -> LinkText<C> {
    LinkText {
        link,
        title,
        content: Some(content),
    }
}

impl<C: Html + 'static> Html for LinkText<C> {
    fn is_inline(&self, context: &mut dyn RenderContext) -> bool {
        if let Some(content) = &self.content {
            content.is_inline(context)
        } else {
            let link_text = context.resolve_link_title(&self.link);
            HtmlPlaintext(link_text).is_inline(context)
        }
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        let (link_text, href) = context.resolve_link(&self.link, context.current_page());
        element("a")
            .attribute_opt("title", self.title)
            .attribute("href", href.clone())
            .content_opt({
                if self.content.is_none() {
                    Some(html_text(link_text))
                } else {
                    None
                }
            })
            .content_opt(self.content)
            .inline(true)
            .build(context, html_out, format)
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct LinkButton(Link);

impl Html for LinkButton {
    fn is_inline(&self, _context: &mut dyn RenderContext) -> bool {
        false
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        let (title, href) = context.resolve_link(&self.0, context.current_page());
        fn style() -> CSSRule {
            (CSSQuery::None, ".link-button", Box::new([
                "display: flex",
                "align-items: center",
                "text-decoration: none",
                "padding: 0.5rem",
                "box-sizing: border-box",
            ]))
        }
        context.stylesheet().register(style);
        element("a")
            .attribute("class", "link-button")
            .attribute("href", href.clone())
            .content(html_text(title))
            .inline(true)
            .build(context, html_out, format)
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

#[derive(Debug)]
pub struct Image {
    source: Link,
    alt_text: String,
    title: Option<String>,
}

impl Html for Image {
    fn is_inline(&self, _context: &mut dyn RenderContext) -> bool {
        true
    }

    fn build(self, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        let href = context.resolve_href(&self.source, context.current_page());

        element("img")
            .inline(true)
            .attribute("src", href)
            .attribute("alt", self.alt_text)
            .attribute_opt("title", self.title)
            .build(context, html_out, format)
    }

    fn build_boxed(self: Box<Self>, context: &mut dyn RenderContext, html_out: &mut dyn Write, format: HtmlFormat) -> std::io::Result<()> {
        self.build(context, html_out, format)
    }
}

pub enum HeadingDepth {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

pub fn html_heading<C: Html + 'static>(depth: usize, content: C) -> HtmlElement {
    let tag_name = match depth {
        1 => "h1",
        2 => "h2",
        3 => "h3",
        4 => "h4",
        5 => "h5",
        6 => "h6",
        _ => panic!("illegal heading depth: {}", depth)
    };

    element(tag_name)
        .attribute("class", "heading")
        .content(content)
}

component!(html_list, [], fn(items: Vec<Box<dyn Html>>, ordered: bool, start: Option<u32>) {
    element(if ordered { "ol" } else { "ul" })
    .content(
        items.vec_map(|item| element("li").content([item]))
    )
    .attribute_opt("start", if ordered { start.map(u32::display_string) } else { None })
}, [
    "display: flex",
    "flex-direction: column"
]);

component!(content_column, [], [
    "display: flex",
    "flex-direction: column",
    "align-items: center",
    "gap: 1rem",
    "box-sizing: border-box",
    "width: 100%"
]);

component!(title, [font_title], fn(name: String) { html_text(name) }, [
    "font-size: min(5rem, 12.5VW)",
    "background: var(--colour-primary)",
    "padding: 1rem"
]);

pub fn footnote_ref(footnote_id: &str, text: &str) -> Component {
    Component {
        content: element("sup")
            .inline(true)
            .attribute("id", format!("footnote_ref-{}", footnote_id))
            .attribute("class", "footnote_ref")
            .content(
                html_link(
                    Link::Custom {
                        link_title: text.to_string(),
                        destination: HRef(format!("#footnote-{}", footnote_id)),
                    },
                    Some(format!("footnote {}", footnote_id)),
                )
            ),
        style: vec![
            || (CSSQuery::None, "cite .footnote_ref, i .footnote_ref", Box::new([
                "margin-left: 0.125rem",
            ]))
        ],
    }
}

pub fn footnote<C: Html + 'static>(footnote_id: &str, text: &str, content: C) -> Component {
    fn style() -> CSSRule {
        (CSSQuery::None, ".footnote", Box::new([
            "font-size: 0.75rem",
        ]))
    }

    Component {
        content: element("div")
            .attribute("id", format!("footnote-{}", footnote_id))
            .attribute("class", "footnote")
            .content((
                html_horizontal_rule(),
                html_text(text),
                html_link(
                    Link::Custom {
                        link_title: "↵".to_string(),
                        destination: HRef(format!("#footnote_ref-{}", footnote_id)),
                    },
                    Some("Return to post".to_string()),
                ),
                content
            )),
        style: vec![style],
    }
}

pub fn image_box(source: Link, alt_text: String, title: Option<String>) -> Component {
    Component {
        content: element("div")
            .attribute("class", "image-box")
            .content(Image { source, alt_text, title }),
        style: vec![
            || (CSSQuery::None, ".image-box", Box::new([
                "display: flex",
                "flex-direction: column",
                "align-items: flex-start",
                "align-self: center",
                "background: var(--colour-secondary)",
                "border: 0.25rem solid var(--colour-secondary-border)",
                "max-width: 100%"
            ])),
            || (CSSQuery::None, ".image-box img", Box::new(["width: 100%"])),
        ],
    }
}

pub fn tab_box<C: Html + 'static>(tabs: Vec<(String, C)>) -> Component {
    fn box_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_bar", Box::new([
            "display: flex",
            "flex-direction: row",
            "justify-content: space-around",
            "align-self: center",
            "min-width: 15rem"
        ]))
    }
    fn button_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_button", Box::new([
            "border: none",
            "background: var(--colour-secondary)",
            "color: var(--text-color)",
            "padding: 0.5rem",
            "border: 0.25rem solid transparent",    // Fake border for layout reasons
            "border-bottom: none",
            "font-size: 1rem",  // <button> has lower font size than usual
        ]))
    }
    fn button_hover_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_button:hover", Box::new([
            "background: var(--colour-secondary-highlight)",
            "cursor: pointer"
        ]))
    }
    fn button_selected_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_button_selected", Box::new([
            "border: 0.25rem solid var(--colour-secondary-border)",
            "border-bottom: none",
            "box-sizing: border-box",
            "background: var(--colour-secondary-highlight)",
            "cursor: default !important"   // Suppress hover behaviour
        ]))
    }
    fn container_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_container", Box::new([
            "visibility: hidden",
            "display: none"
        ]))
    }
    fn container_selected_style() -> CSSRule {
        (CSSQuery::None, ".tab-box_container_selected", Box::new([
            "visibility: visible",
            "display: block"
        ]))
    }

    let mut tab_bar = element("div")
        .attribute("class", "tab-box_bar");

    let mut tab_content = element("div")
        .attribute("class", "tab-box_content");

    let mut first = true;
    for (tab_name, content) in tabs {
        let tab_id = util::next_unique_id();
        tab_bar = tab_bar.content(
            element("button")
                .attribute("class", if first {
                    "tab-box_button tab-box_button_selected"
                } else {
                    "tab-box_button"
                })
                .attribute("id", format!("tab-box_button:{}", tab_id))
                .content(html_text(tab_name))
        );

        tab_content = tab_content.content(
            element("div")
                .attribute("class", if first {
                    "tab-box_container tab-box_container_selected"
                } else {
                    "tab-box_container"
                })
                .attribute("id", format!("tab-box_container:{}", tab_id))
                .content(content)
        );

        first = false;
    }

    Component {
        content: element("div")
            .attribute("class", "tab-box")
            .content(tab_bar)
            .content(tab_content),
        style: vec![box_style, button_style, button_hover_style, button_selected_style, container_style, container_selected_style],
    }
}

pub fn code_box<C: Html + 'static>(title: Option<String>, info: Option<String>, fold: bool, preformatted: bool, content: C) -> Component {
    fn box_style() -> CSSRule {
        (CSSQuery::None, ".code-box", Box::new([
            "display: flex",
            "flex-direction: column",
            "align-items: flex-start",
            "align-self: center",
            "background: var(--colour-secondary)",
            "border: 0.25rem solid var(--colour-secondary-border)",
            "min-width: 15rem"
        ]))
    }
    fn top_style() -> CSSRule {
        (CSSQuery::None, ".code-box_top", Box::new([
            "display: flex",
            "justify-content: space-between",
            "flex-direction: row",
            "width: 100%"
        ]))
    }
    fn title_style() -> CSSRule {
        (CSSQuery::None, ".code-box_title", Box::new([
            "display: flex",
            "border: 0.25rem solid var(--colour-secondary-border)",
            "border-top: none",
            "border-left: none",
            "border-bottom-right-radius: 1rem",
            "padding: 0.25rem",
            "padding-top: 0.1rem"
        ]))
    }
    fn info_style() -> CSSRule {
        (CSSQuery::None, ".code-box_info", Box::new([
            "display: flex",
            "justify-content: center",
            "min-width: 0.75rem",
            "margin-left: auto",
            "border: 0.25rem solid var(--colour-secondary-border)",
            "border-top: none",
            "border-right: none",
            "border-bottom-left-radius: 1rem",
            "padding: 0.25rem",
            "padding-top: 0.1rem"
        ]))
    }
    fn code_style() -> CSSRule {
        (CSSQuery::None, ".code-box code, .code-box pre", Box::new([
            "padding: 0.25rem"
        ]))
    }
    fn code_folded_style() -> CSSRule {
        (CSSQuery::Media("not print"), ".code-box_fold", Box::new([
            "max-height: 15rem",
            "overflow-y: scroll"
        ]))
    }
    fn fold_button_style() -> CSSRule {
        (CSSQuery::None, ".code-box_fold_button", Box::new([
            "display: flex",
            "justify-content: center",
            "align-items: center",
            "color: var(--text-color)",
            "background: var(--colour-secondary)",
            "border: 0.25rem solid var(--colour-secondary-border)",
            "border-top: none",
            "margin-left: auto",
            "margin-right: auto",
            "font-size: 1rem",  // <button> has lower font size than usual
        ]))
    }
    fn fold_button_hover_style() -> CSSRule {
        (CSSQuery::None, ".code-box_fold_button:hover", Box::new([
            "background: var(--colour-secondary-highlight)",
            "cursor: pointer"
        ]))
    }

    let code_block = if preformatted {
        html_preformatted(html_code(content))
    } else {
        html_code(content)
    }
        .attribute_opt("class",if fold {
            Some("code-box_fold")
        } else {
            None
        });

    Component {
        content: element("div")
            .attribute("class", "code-box")
            .content_opt(if title.is_some() || info.is_some() {
                Some(
                    element("div")
                        .attribute("class", "code-box_top")
                        .content_opt(title.map(|title| {
                            element("div")
                                .attribute("class", "code-box_title")
                                .content(html_text(title))
                        }))
                        .content_opt(if fold {
                            Some(
                                element("button")
                                    .attribute("class", "code-box_fold_button")
                                    .attribute("title", "expand")
                                    .content(html_text("⇅"))
                            )
                        } else {
                            None
                        })
                        .content_opt(info.map(|info| {
                            element("div")
                                .attribute("class", "code-box_info")
                                .attribute("title", "info")
                                .content(html_link_content(Link::ID(info), Some("Info".to_string()), html_text("ℹ")))
                        }))
                )
            } else {
                None
            })
            .content(code_block),
        style: vec![box_style, top_style, title_style, info_style, code_style, code_folded_style, fold_button_style, fold_button_hover_style],
    }
}

#[derive(Debug, Clone)]
pub enum NavigationItem {
    SingleLink(Link),
    Tree(Link, Vec<(Link, Vec<Link>)>),
}

pub fn navigation_menu(items: Vec<NavigationItem>) -> Component {
    Component {
        content: element("nav")
            .attribute("class", "navigation font_head content_wide")
            .content(
                vec![
                    element("button")
                        .attribute("class", "link-button")
                        .attribute("id", "navigation-hamburger")
                        .inline(true)
                        .content(html_text("≡"))
                ].extend_chain(
                    items.into_iter().map(|item| match item {
                        NavigationItem::SingleLink(link) => {
                            element("div")
                                .attribute("class", "navigation-menu")
                                .content(LinkButton(link))
                        }
                        NavigationItem::Tree(button, dropdown) => {
                            element("div")
                                .attribute("class", "navigation-menu navigation-hover")
                                .content((
                                    LinkButton(button),
                                    element("div")
                                        .attribute("class", "navigation-dropdown")
                                        .content(
                                            dropdown.vec_map(|(head, items)| {
                                                element("div")
                                                    .attribute("class", "navigation-dropdown-column")
                                                    .content((LinkButton(head), items.vec_map(LinkButton)))
                                            })
                                        )
                                ))
                        }
                    })
                )
            ),
        style: vec![
            || (CSSQuery::None, ".navigation", Box::new([
                "display: flex",
                "flex-direction: row",
                "flex-wrap: wrap",
                "justify-content: space-evenly",
                "align-items: flex-start",
                "gap: 0.5em"
            ])),
            || (CSSQuery::Media("print"), ".navigation", Box::new([
                "visibility: hidden",
                "display: none"
            ])),
            || (CSSQuery::None, ".navigation-menu", Box::new([
                "flex-grow: 1",
                "position: relative",
                "background: var(--colour-primary)",
            ])),
            || (CSSQuery::None, ".navigation-menu a", Box::new([
                "display: flex",
                "flex-direction: row",
                "justify-content: center",
                "height: 1.5rem",
                "width: 100%",
            ])),
            || (CSSQuery::None, ".navigation-menu:hover", Box::new([
                "background: var(--colour-primary-highlight)",
            ])),
            || (CSSQuery::Media("(hover: none) or (max-width:768px)"), ".navigation-menu", Box::new([
                "background: var(--colour-primary-highlight)"
            ])),
            || (CSSQuery::None, "#navigation-hamburger", Box::new([
                "display: none",
                "border: none",
                "background: var(--colour-primary)",
                "color: var(--text-color)",
                "align-items: center",
                "justify-content: center",
                "font-size: 1rem",  // <button> has lower font size than usual
                "width: 1.5rem",
                "height: 1.5rem",
            ])),
            || (CSSQuery::Media("(hover: none) or (max-width:768px)"), "#navigation-hamburger", Box::new([
                "display: flex"
            ])),
            || (CSSQuery::None, "#navigation-hamburger.hamburger-open", Box::new([
                "content: '╳'"
            ])),
            || (CSSQuery::None, "#navigation-hamburger:hover", Box::new([
                "background: var(--colour-primary-highlight)",
                "cursor: pointer"
            ])),
            || (CSSQuery::None, ".navigation-dropdown", Box::new([
                "visibility: hidden",
                "position: absolute",
                "display: flex",
                "flex-direction: row",
                "justify-content: space-evenly",
                "align-items: center",
                "flex-wrap: wrap",
                "background: var(--colour-primary)",
                "width: 100%",
                "box-sizing: border-box",
                "border: 0.125rem solid var(--colour-primary-border)",
                "border-top: 0"
            ])),
            || (CSSQuery::Media("(hover: hover) and (min-width:768px)"), ".navigation-hover:hover .navigation-dropdown", Box::new([
                "visibility: visible",
            ])),
            || (CSSQuery::None, ".navigation-dropdown-column", Box::new([
                "display: flex",
                "flex-grow: 1",
                "flex-direction: column"
            ])),
            || (CSSQuery::None, ".navigation-dropdown-column a", Box::new([
                "justify-content: start",
                "padding-top: 0",
                "padding-bottom: 0"
            ])),
            || (CSSQuery::None, ".navigation-dropdown-column a:hover", Box::new([
                "background: var(--colour-primary-highlight)",
            ])),
            || (CSSQuery::None, ".navigation-dropdown-column a:not(:first-of-type):not(:last-of-type)::before", Box::new([
                "content: '┣'",
                "margin-right: 0.25rem"
            ])),
            || (CSSQuery::None, ".navigation-dropdown-column a:last-of-type::before", Box::new([
                "content: '┗'",
                "margin-right: 0.25rem"
            ])),
        ],
    }
}

pub fn blogpost<C: Html + 'static>(content: C) -> Component {
    fn style() -> CSSRule {
        (CSSQuery::None, ".blogpost", Box::new([
            "display: flex",
            "flex-direction: column"
        ]))
    }

    fn post_content_style() -> CSSRule {
        (CSSQuery::None, ".blogpost > :not(h1,h2,h3,h4,h5,h6)", Box::new([
            "margin-left: 1rem"
        ]))
    }

    Component {
        content: element("div")
            .attribute("class", concat!( stringify!( blogpost ), ))
            .content(content),
        style: vec![style, post_content_style],
    }
}

pub struct PostListEntry<'a> {
    pub(crate) post_id: &'a str,
    pub(crate) post_date: &'a DateTime<Utc>,
    pub(crate) post_title: &'a str,
    // TODO: Maybe category?
}

component!(postlist, [], fn(post_list: Vec<PostListEntry>) {
    (
        html_text("Posts"),
        if post_list.len() > 0 {
            element("lo").content(
                post_list.vec_map(|PostListEntry { post_id, post_date, post_title }| {
                    element("li")
                        .content(LinkText {
                            link: Link::ID(post_id.to_string()),
                            title: None,
                            content: Some(html_text({
                                format!("{} - {}", post_date.format("%Y-%b-%d"), post_title)
                            }))
                        })
                })
            )
        } else {
            html_italics(html_text("No posts here :("))
        }
    )
}, [
    "display: flex",
    "flex-direction: column"
]);

component!(contentbox, [font_text, content_wide], [
    "font-size: 1rem",
    "display: flex",
    "flex-direction: column",
    "gap: 1rem",
    "background: var(--colour-primary)",
    "padding: 1rem"
]);

pub fn content_bottom_spacer() -> Component {
    Component {
        content: element("div")
            .attribute("class", concat!( stringify!( content_bottom_spacer ), )),
        style: vec![|| (
            CSSQuery::None,
            concat!( ".", stringify!( content_bottom_spacer ) ),
            Box::new([
                "width: 100%",
                "height: 16rem",
            ])
        )],
    }
}

pub fn page<B: Html + 'static>(stylesheet: HRef, scripts: Vec<HRef>, lang: &Language, title: String, no_robots: bool, body: B) -> impl Html {
    [
        element("!DOCTYPE")
            .attribute("html", ()),
        element("html")
            .attribute("lang", lang.as_rfc5646_tag())
            .content([
                element("head")
                    .content(
                        vec![
                            element("meta")
                                .attribute("charset", "UTF-8"),
                            element("title")
                                .content(html_text(title)),
                            element("meta")
                                .attribute("name", "robots")
                                .attribute("content", if no_robots { "none" } else { "all" }),
                            element("meta")
                                .attribute("name", "viewport")
                                .attribute("content", "width=device-width, initial-scale=1"),
                            element("link")
                                .attribute("href", stylesheet)
                                .attribute("rel", "stylesheet"),
                        ].extend_chain(scripts.into_iter().map(|href| {
                            element("script")
                                .attribute("src", href)
                                .attribute("defer", ())
                        }))
                    ),
                element("body")
                    .content(body)
            ])
    ]
}
