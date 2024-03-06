use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use chrono::{DateTime};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use crate::blog_post::{BlogPost, Published};
use crate::util::{Language};
use crate::web::component::{content_bottom_spacer, content_column, contentbox, html_heading, html_text, navigation_menu, NavigationItem, page, postlist, PostListEntry, title};
use crate::web::{HRef, Link, PageRef, Renderable, RenderContext, ResourceRender, SpecialCaseRender};
use crate::web::css::CSSBuilder;
use crate::web::html::{Html};
use crate::website_resource::{Resource};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    pub(crate) id_string: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) sub_categories: Vec<Category>,
    #[serde(default)]
    pub(crate) unlisted: bool
}

impl Category {
    fn iter_recurse(&self) -> Box<dyn Iterator<Item=&Category> + '_> {
        Box::new(
            std::iter::once(self)
                .chain(self.sub_categories.iter().flat_map(Category::iter_recurse))
        )
    }

    fn load_map(&self, category_map: &mut HashMap<String, Vec<String>>, path: &mut Vec<String>) -> Result<(), String> {
        path.push(self.id_string.clone());
        category_map.insert(
            self.id_string.clone(),
            path.clone(),
        );
        for sub_category in &self.sub_categories {
            sub_category.load_map(category_map, path)?;
        }
        path.truncate(path.len() - 1);
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum FileName {
    ID,
    Index,
    Resource,
    Custom(&'static str),
}

#[derive(Debug)]
pub enum Document {
    HTML(HtmlDocument),
    Feed(FeedDocument),
    Css(CSSDocument),
    Resource(ResourceDocument),
}

impl Document {
    pub fn id(&self) -> &str {
        match self {
            Document::HTML(HtmlDocument { id, .. }) => &*id,
            Document::Feed(FeedDocument { id, .. }) => &*id,
            Document::Css(CSSDocument { id, .. }) => &*id,
            Document::Resource(ResourceDocument { resource, .. }) => &*resource.id
        }
    }

    pub fn title(&self) -> Option<&str> {
        match self {
            Document::HTML(html) => Some(&*html.title),
            Document::Feed(_) => None,
            Document::Css(_) => None,
            Document::Resource(_) => None
        }
    }

    pub fn filename(&self) -> FileName {
        match self {
            Document::HTML(HtmlDocument { filename, .. }) => *filename,
            Document::Feed(FeedDocument { filename, .. }) => *filename,
            Document::Css(CSSDocument { filename, .. }) => *filename,
            Document::Resource(ResourceDocument { filename, .. }) => *filename
        }
    }

    pub fn extension(&self) -> &str {
        match self {
            Document::HTML(_) => ".html",
            Document::Feed(_) => ".rss",
            Document::Css(_) => ".css",
            Document::Resource(doc) => doc.resource.resource_type.extension()
        }
    }

    pub fn category(&self) -> Option<&str> {
        match self {
            Document::HTML(HtmlDocument { category, .. }) => category.as_deref(),
            Document::Feed(FeedDocument { category, .. }) => category.as_deref(),
            Document::Css(_) => None,
            Document::Resource(_) => None
        }
    }

    pub fn build(self, context: &dyn RenderContext) -> Box<dyn Renderable> {
        match self {
            Document::HTML(mut html) => html.render.take()
                .expect("double-render")
                .call_once((context, &html)),
            Document::Feed(mut feed) => feed.render.take()
                .expect("double-render")
                .call_once((context, &feed)),
            Document::Css(mut css) => css.render.take()
                .expect("double-render")
                .call_once((context, &css)),
            Document::Resource(mut script) => script.render.take()
                .expect("double-render")
                .call_once((context, &script))
        }
    }
}

impl From<HtmlDocument> for Document {
    fn from(value: HtmlDocument) -> Self {
        Document::HTML(value)
    }
}

impl From<FeedDocument> for Document {
    fn from(value: FeedDocument) -> Self {
        Document::Feed(value)
    }
}

impl From<CSSDocument> for Document {
    fn from(value: CSSDocument) -> Self {
        Document::Css(value)
    }
}

impl From<ResourceDocument> for Document {
    fn from(value: ResourceDocument) -> Self {
        Document::Resource(value)
    }
}

pub struct HtmlDocument {
    id: String,
    title: String,
    filename: FileName,
    category: Option<String>,
    render: Option<Box<dyn FnOnce(&dyn RenderContext, &HtmlDocument) -> Box<dyn Renderable>>>,
}

impl HtmlDocument {
    pub fn page_ref(&self) -> PageRef {
        PageRef(&*self.id)
    }

    pub fn new<R: FnOnce(&dyn RenderContext, &HtmlDocument) -> Box<dyn Renderable> + 'static>(id: String, title: String, filename: FileName, category: Option<String>, render: R) -> Self {
        Self { id, title, filename, category, render: Some(Box::new(render)) }
    }
}

impl Debug for HtmlDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlDocument")
            .field("id", &self.id)
            .field("title", &self.title)
            .field("category", &self.category)
            .field("filename", &self.filename)
            .finish()
    }
}

pub struct FeedDocument {
    id: String,
    title: String,
    filename: FileName,
    category: Option<String>,
    render: Option<Box<dyn FnOnce(&dyn RenderContext, &FeedDocument) -> Box<dyn Renderable>>>,
}

impl FeedDocument {
    pub fn new<R: FnOnce(&dyn RenderContext, &FeedDocument) -> Box<dyn Renderable> + 'static>(id: String, title: String, filename: FileName, category: Option<String>, render: R) -> Self {
        Self { id, title, filename, category, render: Some(Box::new(render)) }
    }
}

impl Debug for FeedDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FeedDocument")
            .field("id", &self.id)
            .field("category", &self.category)
            .field("filename", &self.filename)
            .finish()
    }
}

pub struct CSSDocument {
    id: String,
    filename: FileName,
    render: Option<Box<dyn FnOnce(&dyn RenderContext, &CSSDocument) -> Box<dyn Renderable>>>,
}

impl CSSDocument {
    pub fn new<R: FnOnce(&dyn RenderContext, &CSSDocument) -> Box<dyn Renderable> + 'static>(id: String, filename: FileName, render: R) -> Self {
        Self { id, filename, render: Some(Box::new(render)) }
    }
}

impl Debug for CSSDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CSSDocument")
            .field("id", &self.id)
            .field("filename", &self.filename)
            .finish()
    }
}

pub struct ResourceDocument {
    resource: Resource,
    filename: FileName,
    render: Option<Box<dyn FnOnce(&dyn RenderContext, &ResourceDocument) -> Box<dyn Renderable>>>,
}

impl ResourceDocument {
    pub fn new<R: FnOnce(&dyn RenderContext, &ResourceDocument) -> Box<dyn Renderable> + 'static>(resource: Resource, filename: FileName, render: R) -> Self {
        Self { resource, filename, render: Some(Box::new(render)) }
    }
}

impl Debug for ResourceDocument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResourceDocument")
            .field("id", &self.resource.id)
            .field("filename", &self.filename)
            .finish()
    }
}

impl Document {
    pub fn page_ref(&self) -> PageRef {
        PageRef(self.id())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Website {
    pub title: String,
    pub description: String,
    pub categories: Vec<Category>,
    #[serde(skip, default = "IndexMap::new")]
    pub posts: IndexMap<String, BlogPost>,
    #[serde(skip, default = "Vec::new")]
    pub resources: Vec<Resource>,
}

impl Website {
    fn validate(&self) {
        for category in &self.categories {
            for subcategory in &category.sub_categories {
                for sub_sub_category in &subcategory.sub_categories {
                    if sub_sub_category.sub_categories.len() > 0 {
                        panic!("Category {} nests beyond level 3", sub_sub_category.id_string);
                    }
                }
            }
        }
    }

    pub fn nav_items(&self) -> Vec<Link> {
        let mut items = vec![Link::ID("home".to_string())];
        for category in self.categories.iter() {
            items.push(Link::ID(category.id_string.clone()))
        }
        items
    }

    fn render_page<C: Html + 'static>(context: &dyn RenderContext, document: &HtmlDocument, navigation: Vec<NavigationItem>, content: C) -> Box<dyn Renderable> {
        let mut content_items: Vec<Box<dyn Html>> = vec![Box::new(title(context.title().to_string()))];
        content_items.push(Box::new(navigation_menu(navigation)));
        content_items.push(Box::new(content));
        content_items.push(Box::new(content_bottom_spacer()));
        Box::new(page(
            context.stylesheet_link(PageRef(&*document.id)),
            context.global_scripts(document.page_ref()),
            &Language::English,
            {
                if let Some(title_prefix) = context.title_prefix() {
                    title_prefix.to_string() + " - " + &*document.title
                } else {
                    document.title.clone()
                }
            },
            true,
            content_column(content_items),
        ))
    }

    fn documents(&self) -> Vec<Document> {
        let mut documents: Vec<Document> = Vec::new();

        let mut category_parents = HashMap::<String, Option<String>>::new();
        let mut category_children = HashMap::<String, HashSet<String>>::new();
        fn load_category_parents(map: &mut HashMap<String, Option<String>>, category: &Category, parent: Option<&str>) {
            map.insert(category.id_string.clone(), parent.map(str::to_string));
            for sub_category in &category.sub_categories {
                load_category_parents(map, sub_category, Some(&*category.id_string));
            }
        }

        fn load_category_children(map: &mut HashMap<String, HashSet<String>>, category: &Category, parent: Option<&str>) {
            map.entry(category.id_string.clone())
                .or_insert_with(HashSet::new)
                .insert(category.id_string.clone());

            if let Some(parent) = parent {
                map.entry(parent.to_string())
                    .or_insert_with(HashSet::new)
                    .insert(category.id_string.clone());
            }

            for sub_category in &category.sub_categories {
                load_category_children(map, sub_category, Some(&*category.id_string));
            }
            if parent.is_some() {
                for sub_category in &category.sub_categories {
                    load_category_children(map, sub_category, parent);
                }
            }
        }

        for category in &self.categories {
            load_category_parents(&mut category_parents, category, None);
            load_category_children(&mut category_children, category, None)
        }

        let mut navigation = Vec::new();
        navigation.push(NavigationItem::SingleLink(Link::ID("home".to_string())));
        for category in &self.categories {
            let mut tree = Vec::new();
            for head_category in category.sub_categories.iter().filter(|c| !c.unlisted) {
                tree.push((
                    Link::ID(head_category.id_string.clone()),
                    head_category.sub_categories.iter().map(|category| Link::ID(category.id_string.clone())).collect()
                ))
            }
            if tree.len() > 0 {
                navigation.push(NavigationItem::Tree(Link::ID(category.id_string.clone()), tree))
            } else {
                navigation.push(NavigationItem::SingleLink(Link::ID(category.id_string.clone())))
            }
        }


        let home_nav = navigation.clone();
        let description = self.description.clone();
        documents.push(
            HtmlDocument::new(
                "home".to_string(),
                "Home".to_string(),
                FileName::Index,
                None,
                move |ctx, document| {
                    Website::render_page(ctx, document, home_nav, contentbox(html_text(description)))
                },
            ).into()
        );
        documents.push(
            CSSDocument::new(
                "stylesheet".to_string(),
                FileName::ID,
                |_, _| Box::new(SpecialCaseRender()),
            ).into()
        );

        for resource in &self.resources {
            documents.push(
                ResourceDocument::new(
                    resource.clone(),
                    FileName::Resource,
                    |_, document| Box::new(ResourceRender(document.resource.path.clone())),
                ).into()
            )
        }

        for category in self.categories.iter().flat_map(Category::iter_recurse).filter(|category| !category.unlisted) {
            let category_nav = navigation.clone();
            let description = category.description.clone();

            let mut content: Vec<Box<dyn Html>> = vec![
                Box::new(html_heading(1, html_text(&category.title))),
                Box::new(html_text(description)),
            ];

            if category_children.get("blog").unwrap().contains(&category.id_string) {
                let post_categories = category_children.get(&category.id_string).unwrap();
                let list: Vec<PostListEntry> = self.posts.iter()
                    .filter(|(_, post)| post_categories.contains(&post.metadata.category))
                    .filter(|(_, post)| post.metadata.published == Published::True) // Ignore unpublished or unlisted posts
                    .map(|(id, post)| {
                        PostListEntry {
                            post_id: id,
                            post_date: &post.metadata.date,
                            post_title: &post.metadata.title,
                        }
                    })
                    .collect::<Vec<_>>();

                content.push(Box::new(postlist(list)))
            }

            documents.push(
                HtmlDocument::new(
                    category.id_string.clone(),
                    category.title.clone(),
                    FileName::Index,
                    Some(category.id_string.clone()),
                    move |ctx, document| {
                        Website::render_page(ctx, document, category_nav, contentbox(content))
                    },
                ).into()
            );
        }

        for (post_id, post) in &self.posts {
            let post = post.clone();
            let post_nav = navigation.clone();
            documents.push(
                HtmlDocument::new(
                    post_id.clone(),
                    post.metadata.title.clone(),
                    FileName::ID,
                    Some(post.metadata.category.clone()),
                    move |ctx, document| {
                        Website::render_page(ctx, document, post_nav.clone(), contentbox(post.render_content(ctx)))
                    },
                ).into()
            );
        }

        documents
    }

    pub fn build(mut self, stylesheet: CSSBuilder) -> Result<WebsiteBuilder, String> {
        self.validate();

        self.posts.sort_by(|_, left, _, right| DateTime::cmp(&left.metadata.date, &right.metadata.date).reverse());

        let mut category_map = HashMap::<String, Vec<String>>::new();
        let mut path_vec = Vec::new();
        for category in &self.categories {
            category.load_map(&mut category_map, &mut path_vec)?;
        }

        let mut id_set = HashSet::new();
        let mut routes = HashMap::<String, Vec<String>>::new();
        let mut route_set = HashSet::<Vec<String>>::new();

        for document in self.documents() {
            if !id_set.insert(document.id().to_string()) {
                Err(format!("duplicate document ID: {}", document.id()))?;
            }

            let mut route = Vec::new();
            if let Some(category) = document.category() {
                route = category_map.get(category).ok_or(format!("invalid document category:{:?} {}", document, category))?.clone();
            }

            match document.filename() {
                FileName::ID => route.push(document.id().to_string() + document.extension()),
                FileName::Index => route.push("index".to_string() + document.extension()),
                FileName::Resource => {
                    route.push("rsc".to_string());
                    let id = document.id();
                    route.push(id.strip_prefix("resource:").unwrap_or(id).to_string() + document.extension());
                },
                FileName::Custom(filename) => route.push(filename.to_string() + document.extension()),
            }

            let document_duplicate = routes.insert(document.id().to_string(), route.clone()).is_some();
            let route_duplicate = !route_set.insert(route.clone());

            if document_duplicate {
                return Err(format!("duplicate document: {}", document.page_ref()));
            } else if route_duplicate {
                let collided_pages = routes.iter()
                    .filter(|(_, map_route)| *map_route == &route)
                    .map(|(id, _)| id.clone())
                    .collect::<Vec<String>>();
                return Err(format!("duplicate route: {} for {:?}", route.join("/"), collided_pages));
            };
        }

        // Routes valid from here
        let documents = self.documents();

        let context = WebsiteRenderContext {
            title: self.title,
            current_page: None,
            document_titles: HashMap::from_iter(documents.iter().filter_map(|document| document.title().map(|title| (document.id().to_string(), title.to_string())))),
            stylesheet,
            global_scripts: documents.iter().filter_map(|document| {
                if let Document::Resource(script) = document && script.resource.resource_type.is_global_script() {
                    Some(Link::ID(script.resource.id.clone()))
                } else {
                    None
                }
            }).collect(),
            stylesheet_link: Link::ID("stylesheet".to_string()),
            routes,
            categories: self.categories
        };

        return Ok(WebsiteBuilder::new(context, documents));
    }
}

pub struct WebsiteRenderContext {
    title: String,
    current_page: Option<String>,
    stylesheet: CSSBuilder,
    stylesheet_link: Link,
    document_titles: HashMap<String, String>,
    global_scripts: Vec<Link>,
    categories: Vec<Category>,
    routes: HashMap<String, Vec<String>>,
}

impl WebsiteRenderContext {
    pub fn route(&self, page_ref: PageRef) -> Option<&Vec<String>> {
        self.routes.get(page_ref.0)
    }

    fn set_page(&mut self, page_id: &str) {
        self.current_page = Some(page_id.to_string());
    }
}

impl RenderContext for WebsiteRenderContext {
    fn title(&self) -> &str {
        &self.title
    }

    fn title_prefix(&self) -> Option<&str> {
        Some(&self.title)
    }

    fn resolve_href(&self, link: &Link, from_page: PageRef) -> HRef {
        match link {
            Link::ID(id) => {
                let from = self.routes.get(from_page.0).expect(&*format!("invalid page reference: {}", from_page));
                let to = self.routes.get(id).expect(&*format!("invalid page reference: {}", id));

                let mut route = String::new();
                let start_index = from.iter().zip(to).take_while(|(a, b)| a == b).count();

                if start_index == to.len() { // Special case
                    route = format!("./{}", to.last().expect("link to empty route"));
                } else {
                    for _ in 1..(from.len() - start_index) {
                        route += "../"
                    }
                    route += &*to[start_index..].join("/");
                }
                HRef(route)
            }
            Link::Custom { destination, .. } => destination.clone()
        }
    }
    fn resolve_link_title(&self, link: &Link) -> String {
        match link {
            Link::ID(id) => self.document_titles.get(id)
                .expect(&*
                    if self.routes.contains_key(id) {
                        format!("Attempt to resolve link to document without title for {:?}", link)
                    } else {
                        format!("Attempt to resolve link to unknown ID for {:?}", link)
                    }
                )
                .clone(),
            Link::Custom { link_title: name, .. } => name.clone()
        }
    }

    fn resolve_link(&self, link: &Link, from_page: PageRef) -> (String, HRef) {
        (self.resolve_link_title(link), self.resolve_href(link, from_page))
    }

    fn resolve_category(&self, category_id: &str) -> &Category {
        self.categories.iter()
            .flat_map(Category::iter_recurse)
            .find(|category| category.id_string == category_id)
            .expect(&*format!("attempt to resolve unknown category `{}`", category_id))
    }


    fn current_page(&self) -> PageRef {
        if let Some(page) = &self.current_page {
            PageRef(&*page)
        } else {
            panic!("Rendering with unset current-page");
        }
    }

    fn stylesheet(&mut self) -> &mut CSSBuilder {
        &mut self.stylesheet
    }

    fn stylesheet_link(&self, for_page: PageRef) -> HRef {
        self.resolve_href(&self.stylesheet_link, for_page)
    }

    fn global_scripts(&self, for_page: PageRef) -> Vec<HRef> {
        self.global_scripts.iter().map(|link| self.resolve_href(&link, for_page)).collect()
    }
}

pub struct WebsiteBuilder {
    context: WebsiteRenderContext,
    documents: std::vec::IntoIter<Document>,
}

impl WebsiteBuilder {
    pub fn new(context: WebsiteRenderContext, documents: Vec<Document>) -> WebsiteBuilder {
        WebsiteBuilder {
            context,
            documents: documents.into_iter(),
        }
    }

    pub fn stylesheet(&mut self) -> &mut CSSBuilder {
        &mut self.context.stylesheet
    }

    pub fn into_stylesheet(self) -> CSSBuilder {
        self.context.stylesheet
    }

    pub fn next(&mut self) -> Option<(&mut WebsiteRenderContext, Document)> {
        if let Some(document) = self.documents.next() {
            self.context.set_page(document.id());
            Some((&mut self.context, document))
        } else {
            None
        }
    }
}
