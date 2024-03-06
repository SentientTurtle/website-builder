use std::collections::HashMap;
use std::path::Path;
use chrono::{Utc};
use markdown::mdast::Node;
use markdown::ParseOptions;
use serde::{Deserialize, Serialize};
use crate::blog_post::code_blocks::{QueryResponse, QueryResponseMulti};
use crate::util::{DisplayExt, VecExt};
use crate::web::component::{blogpost, html_text, html_paragraph, code_box, html_code, html_heading, html_italics, image_box, html_link, html_span, html_blockquote, footnote_ref, html_raw, html_list, html_checkbox, footnote, html_link_content, html_break, html_strong, html_horizontal_rule};
use crate::web::html::{Html};
use crate::web::{HRef, Link, RenderContext};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Published {
    True,
    False,
    Unlisted
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogMeta {
    pub author: String,
    pub category: String,
    #[serde(with = "blog_date_format")]
    pub date: chrono::DateTime<Utc>,
    pub title: String,
    pub published: Published
}

mod blog_date_format {
    // from https://serde.rs/custom-date-format.html
    use chrono::{DateTime, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S %z";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error> where D: Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc))
    }
}

#[derive(Debug, Clone)]
pub struct BlogPost {
    pub metadata: BlogMeta,
    markdown: String,
}

impl BlogPost {
    pub fn render_content(&self, ctx: &dyn RenderContext) -> [Box<dyn Html>; 1] {
        let mut post = markdown::to_mdast(&*self.markdown, &ParseOptions::gfm())
            .expect("post must be valid markdown to pass build_post");
        remove_non_renderable_nodes(&mut post);

        if let Node::Root(root_node) = post {
            let mut post_contents: Vec<Box<dyn Html>> = Vec::new();

            post_contents.push(
                Box::new(html_span(html_italics([
                    Box::new(
                        html_span(html_text(self.metadata.date.format("%Y-%m-%d").to_string()))
                            .attribute("title", "publication date")
                    ) as Box<dyn Html>,
                    Box::new(html_text(" - in ")),
                    {
                        let category = ctx.resolve_category(&*self.metadata.category);
                        if category.unlisted {
                            Box::new(
                                html_span(html_text(&category.title))
                                    .attribute("title", "category")
                            )
                        } else {
                            Box::new(html_link(Link::ID(self.metadata.category.clone()), Some("category".to_string())))
                        }
                    },
                    Box::new(html_text(" - ")),
                    Box::new(
                        html_span(html_text(&*self.metadata.author))
                            .attribute("title", "author")
                    ),
                ])))
            );

            root_node.children.into_iter()
                .map(render)
                .collect_into(&mut post_contents);

            [Box::new(blogpost(post_contents))]
        } else {
            panic!("No root node in markdown {:?}", post);
        }
    }
}

fn remove_non_renderable_nodes(node: &mut Node) {
    if let Some(children) = node.children_mut() {
        children.retain(|child| {
            if let Node::Code(code) = child {
                if let Some(meta) = &code.meta && meta == "blogmeta" {
                    return false;
                }
            }
            true
        });
        for child in children {
            remove_non_renderable_nodes(child)
        }
    }
}

fn retrieve_meta(post: &Node, meta_list: &mut Vec<String>) {
    if let Some(children) = post.children() {
        for node in children {
            if let Node::Code(code) = node {
                if let Some(meta) = &code.meta && meta == "blogmeta" {
                    meta_list.push(code.value.clone());
                }
            }

            retrieve_meta(node, meta_list);
        }
    }
}

pub fn build_post(markdown: String) -> Result<BlogPost, String> {
    let mut post = markdown::to_mdast(&*markdown, &ParseOptions::gfm())
        .map_err(|e| format!("post was not valid markdown {}", e))?;

    let mut meta_list = Vec::new();
    retrieve_meta(&mut post, &mut meta_list);

    if meta_list.len() == 1 {
        let metadata: BlogMeta = serde_yaml::from_str(&*meta_list[0]).map_err(DisplayExt::display_string)?;
        Ok(BlogPost { metadata, markdown })
    } else if meta_list.len() == 0 {
        Err("no blogmeta blocks defined")?
    } else {
        Err("multiple blogmeta blocks defined")?
    }
}

mod code_blocks {
    use serde::{Deserialize, Serialize};
    use crate::web::component::{code_box, html_bold, html_break, html_horizontal_rule, html_text, tab_box};
    use crate::web::html::{Component};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueryResponse {
        q_title: Option<String>,
        query: String,
        r_title: Option<String>,
        response: String,
    }

    impl QueryResponse {
        pub fn render(self, lang: Option<String>, info: Option<String>, fold: bool, preformatted: bool) -> Component {
            match (self.q_title, self.r_title) {
                (Some(q_title), Some(r_title)) => code_box(lang, info, fold, preformatted, (
                    html_bold(html_text(q_title)),
                    html_break(),
                    html_text(self.query),
                    html_horizontal_rule(),
                    html_bold(html_text(r_title)),
                    html_break(),
                    html_text(self.response),
                )),
                (Some(q_title), None) => code_box(lang, info, fold, preformatted, (
                    html_bold(html_text(q_title)),
                    html_break(),
                    html_text(self.query),
                    html_horizontal_rule(),
                    html_text(self.response),
                )),
                (None, Some(r_title)) => code_box(lang, info, fold, preformatted, (
                    html_text(self.query),
                    html_horizontal_rule(),
                    html_bold(html_text(r_title)),
                    html_break(),
                    html_text(self.response),
                )),
                (None, None) => code_box(lang, info, fold, preformatted, (
                    html_text(self.query),
                    html_horizontal_rule(),
                    html_text(self.response),
                ))
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct QueryResponseMulti (
        Vec<(String, QueryResponse)>,
    );

    impl QueryResponseMulti {
        pub fn render(self, lang: Option<String>, info: Option<String>, fold: bool, preformatted: bool) -> Component {
            tab_box(
                self.0.into_iter()
                    .map(|(title, response)| (title, response.render(lang.clone(), info.clone(), fold, preformatted)))
                    .collect()
            )
        }
    }
}

fn render(node: Node) -> Box<dyn Html> {
    match node {
        Node::Root(_) => panic!("Nested root in Markdown nodes!"),
        Node::BlockQuote(blockquote) => Box::new(
            html_blockquote(blockquote.children.vec_map(render))
        ),
        Node::FootnoteDefinition(definition) => Box::new(
            footnote(&definition.identifier, definition.label.as_ref().unwrap_or(&definition.identifier), definition.children.vec_map(render))
        ),
        Node::List(list) => Box::new(
            html_list(list.children.vec_map(render), list.ordered, list.start)
        ),
        // Node::Toml(toml) => {}
        // Node::Yaml(yaml) => {}
        Node::Break(_) => Box::new(
            html_break()
        ),
        Node::InlineCode(inline_code) => Box::new(
            html_code(html_text(inline_code.value))
        ),
        // Node::InlineMath(inline_math) => {}
        // Node::Delete(delete) => {}
        Node::Emphasis(e) => Box::new(
            html_italics(e.children.vec_map(render))
        ),
        Node::FootnoteReference(reference) => Box::new(
            footnote_ref(
                &reference.identifier,
                reference.label.as_deref().unwrap_or(&reference.identifier),
            )
        ),
        Node::Html(html) => Box::new(
            html_raw(html.value)
        ),
        Node::Image(image) => Box::new(
            if image.url.starts_with("../resource") {
                let resource_id = format!(
                    "resource:{}",
                    Path::new(&image.url)
                        .file_stem()
                        .unwrap()
                        .to_string_lossy()
                );

                image_box(Link::ID(resource_id), image.alt, image.title)
            } else {
                panic!("Unknown image url `{}`", image.url);
            }
        ),
        // Node::ImageReference(image_reference) => {}
        Node::Link(link) => Box::new(
            if let Some(id) = link.url.strip_prefix("intralink:") {
                html_link_content(
                    Link::ID(id.to_string()),
                    link.title,
                    link.children.vec_map(render),
                )
            } else {
                html_link_content(
                    Link::Custom {
                        link_title: "".to_string(),
                        destination: HRef(link.url),
                    },
                    link.title,
                    link.children.vec_map(render),
                )
            }
        ),
        // Node::LinkReference(link_reference) => {}
        Node::Strong(s) => Box::new(
            html_strong(s.children.vec_map(render))
        ),
        Node::Text(t) => Box::new(
            html_text(t.value)
        ),
        Node::Code(code) => Box::new(
            if let Some(meta) = code.meta {
                let meta_tags: HashMap<String, Option<String>> = meta.split_ascii_whitespace()
                    .map(|entry| {
                        entry.split_once('=')
                            .map(|(l, r)| (l.to_string(), Some(r.to_string())))
                            .unwrap_or_else(|| (entry.to_string(), None))
                    })
                    .collect();

                let fold = meta_tags.contains_key("fold");
                let preformatted = meta_tags.contains_key("preformatted");
                let info = meta_tags.get("info").map(|opt| opt.as_ref().expect("info without page").clone());

                if let Some(Some(format)) = meta_tags.get("format") {
                    match format.as_str() {
                        "query-response" => {
                            serde_yaml::from_str::<QueryResponse>(&*code.value)
                                .expect("invalid code block yaml query-response")
                                .render(code.lang, info, fold, preformatted)
                        },
                        "query-response-multi" => {
                            serde_yaml::from_str::<QueryResponseMulti>(&*code.value)
                                .expect("invalid code block yaml query-response-multi")
                                .render(code.lang, info, fold, preformatted)
                        },
                        _ => panic!("Unknown code block format: {}", format)
                    }
                } else {
                    code_box(code.lang, info, fold, preformatted, html_text(code.value))
                }
            } else {
                code_box(code.lang, None, false, false, html_text(code.value))
            }
        ),
        // Node::Math(math) => {}
        Node::Heading(h) => Box::new(
            html_heading(h.depth as usize, h.children.vec_map(render))
        ),
        // Node::Table(table) => {}
        Node::ThematicBreak(_thematic_break) => Box::new(html_horizontal_rule()),
        // Node::TableRow(table_row) => {}
        // Node::TableCell(table_cell) => {}
        Node::ListItem(list_item) => {
            if let Some(checked) = list_item.checked {
                Box::new((html_checkbox(checked, false), list_item.children.vec_map(render)))
            } else {
                Box::new(list_item.children.vec_map(render))
            }
        }
        // Node::Definition(definition) => {}
        Node::Paragraph(p) => Box::new(
            html_paragraph(p.children.vec_map(render))
        ),
        _ => panic!("unknown node type: {:?}", node)
    }
}
