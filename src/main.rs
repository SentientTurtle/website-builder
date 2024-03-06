#![feature(let_chains)]
#![feature(fn_traits)]
#![feature(hash_set_entry)]
#![feature(try_blocks)]
#![feature(path_file_prefix)]
#![feature(iter_intersperse)]
#![feature(iter_collect_into)]
#![allow(dead_code)]

use std::ffi::OsStr;
use std::fs::{DirEntry, File};
use std::io;
use std::path::{Path};
use std::time::Instant;
use crate::blog_post::Published;
use crate::website::{Website};
use crate::web::css::{CSSBuilder};
use crate::website_resource::{Resource, ResourceType};

#[macro_use]
mod web;

mod website;

mod blog_post;

mod website_resource {
    use std::path::PathBuf;

    #[derive(Debug, Clone)]
    pub struct Resource {
        pub resource_type: ResourceType,
        pub id: String,
        pub path: PathBuf,
    }

    impl Resource {
        pub fn new(resource_type: ResourceType, name: String, path: PathBuf) -> Self {
            Self { resource_type, id: name, path }
        }
    }

    #[derive(Debug, Copy, Clone)]
    pub enum ResourceType {
        SVG,
        PNG,
        JS { is_global_script: bool },
    }

    impl ResourceType {
        pub fn is_global_script(self) -> bool {
            if let ResourceType::JS { is_global_script } = self {
                is_global_script
            } else {
                false
            }
        }

        pub fn extension(self) -> &'static str {
            match self {
                ResourceType::SVG => ".svg",
                ResourceType::JS { .. } => ".js",
                ResourceType::PNG => ".png"
            }
        }
    }
}

mod util;

#[derive(Debug)]
pub enum BuildError {
    IO(io::Error),
    Serde(serde_json::Error),
    String(String),
}

impl From<io::Error> for BuildError {
    fn from(value: io::Error) -> Self {
        BuildError::IO(value)
    }
}

impl From<serde_json::Error> for BuildError {
    fn from(value: serde_json::Error) -> Self {
        BuildError::Serde(value)
    }
}

impl From<String> for BuildError {
    fn from(value: String) -> Self {
        BuildError::String(value)
    }
}

fn main() {
    let start = Instant::now();

    println!("Starting website build...");
    let mut css = CSSBuilder::new();
    css.import("url('https://fonts.googleapis.com/css2?family=Raleway:wght@100;400&family=Roboto+Mono&display=block')");

    css!(&mut css, Root, [
        "--body-bg: #000000",

        "--colour-primary: #160020",
        "--colour-primary-highlight: #240035",
        "--colour-primary-border: #240035",

        "--colour-secondary: #002020",
        "--colour-secondary-highlight: #003535",
        "--colour-secondary-border: #003535",

        "--text-colour: #ffffff",
        "font-size: 16px"
    ]);

    css!(&mut css, @Media:"print", Root, [
        "--body-bg: white",

        "--colour-primary: white",
        "--colour-primary-highlight: white",
        "--colour-primary-border: black",

        "--colour-secondary: white",
        "--colour-secondary-highlight: white",
        "--colour-secondary-border: black",

        "--text-colour: black",
        "font-size: 12pt"
    ]);

    css!(&mut css, Tag:"body", [
        "display: flex",
        "width: 100%",
        "height: 100%",
        "margin: 0",
        "background: var(--body-bg)",
        "color: var(--text-colour)"
    ]);

    css!(&mut css, Tag:"a", [
        "color: var(--text-colour)"
    ]);

    css!(&mut css, Tag:"pre", [
        "display: flex",
        "white-space: break-spaces",
        "margin: 0"
    ]);

    css!(&mut css, Tag:"li > p", [
        "margin: 0"
    ]);

    css!(&mut css, Tag:"h1, h2, h3, h4, h5, h6", [
        "margin: 0",
        "margin-top: 1rem"
    ]);

    css!(&mut css, Tag:"p", [
        "margin: 0.5rem"
    ]);

    css!(&mut css, Tag:"hr", [
        "width: 100%"
    ]);

    css!(&mut css, Tag:"blockquote", [
        "background: var(--colour-secondary)",
        "border: 0.25rem solid var(--colour-secondary-border)",
        "padding-left: 1rem",
        "padding-right: 1rem"
    ]);

    css!(&mut css, Class:"font_title", [
        "font-family: 'Raleway', sans-serif",
        "font-weight: 100"
    ]);

    css!(&mut css, Class:"font_head", [
        "font-family: 'Raleway', sans-serif",
        "font-weight: 400"
    ]);

    css!(&mut css, Class:"font_text", [
        "font-family: 'Roboto Mono', sans-serif",
        "font-weight: 400"
    ]);

    css!(&mut css, Class:"column", [
        "font-family: 'Roboto Mono', sans-serif",
        "font-weight: 400"
    ]);

    css!(&mut css, Class:"content_wide", [
        "box-sizing: border-box",
        "width: 64rem"
    ]);

    // 64 rem width + 1 rem padding
    css!(&mut css, @Media:"(max-width: 65rem)", Class:"content_wide", [
        "box-sizing: border-box",
        "width: 100%"
    ]);

    let mut website: Website = serde_json::from_reader(File::open("./rsc/website.json").unwrap()).unwrap();

    for entry in std::fs::read_dir("./rsc/posts/").unwrap() {
        let path = entry.unwrap().path();
        if path.extension() == Some(OsStr::new("md")) {
            let post_id = path.file_prefix().unwrap().to_str()
                .ok_or_else(|| format!("post {:?} has non-unicode filename", path)).unwrap();
            println!("\tpost: {:?}", path);
            let post_string = String::from_utf8(std::fs::read(&path).unwrap())
                .map_err(|e| format!("post {:?} was not in UTF8 {}", path, e)).unwrap();

            let post = blog_post::build_post(post_string)
                .map_err(|e| format!("Error during post {:?} {}", path, e)).unwrap();

            if post.metadata.published != Published::False {
                if website.posts.insert(post_id.to_string(), post).is_some() {
                    panic!("duplicate post id {}", post_id);
                }
            } else {
                // Drop post
                continue;
            }
        } else {
            panic!("Unknown post file type: {:?}", path);
        }
    }

    fn load_resource(resource_list: &mut Vec<Resource>, prefix: &Path, entry: DirEntry) -> Result<(), BuildError> {
        let path = entry.path();
        if entry.file_type()?.is_file() {
            let extension = path.extension().unwrap().to_string_lossy();

            let resource_type = match &*extension {
                "svg" => ResourceType::SVG,
                "png" => ResourceType::PNG,
                "js" => ResourceType::JS { is_global_script: path.strip_prefix(prefix).is_ok_and(|sub_path| sub_path.starts_with("global script")) },
                _ => Err(format!("Unknown resource type: {}", extension))?
            };

            let resource_id = format!(
                "resource:{}",
                path.file_stem()
                    .unwrap()
                    .to_string_lossy()
            );
            resource_list.push(Resource::new(resource_type, resource_id, path));
        } else {
            for sub_entry in std::fs::read_dir(path)? {
                load_resource(resource_list, prefix, sub_entry?)?;
            }
        }
        Ok(())
    }

    let resource_dir = Path::new("./rsc/resource/");
    for entry in std::fs::read_dir(resource_dir).unwrap() {
        load_resource(&mut website.resources, resource_dir, entry.unwrap()).unwrap();
    }

    for item in std::fs::read_dir("./out").unwrap() {
        let entry = item.unwrap();
        if entry.file_type().unwrap().is_file() {
            std::fs::remove_file(entry.path()).unwrap();
        } else {
            std::fs::remove_dir_all(entry.path()).unwrap();
        }
    }

    let mut builder = website.build(css).unwrap();

    while let Some((context, document)) = builder.next() {
        if let Some(route) = context.route(document.page_ref()) {
            let directories = "./out/".to_string() + &*route[..route.len() - 1].join("/");
            std::fs::create_dir_all(directories).unwrap();
            let mut html_out = File::create("./out/".to_string() + &*route.join("/"))
                .map_err(|e| format!("error writing file for {:?}: {}", document, e))
                .unwrap();

            document.build(context)
                .render(context, &mut html_out).unwrap();
        } else {
            panic!("Unknown route for: {:?}", document.page_ref());
        }
    }

    let mut css_out = File::create("./out/stylesheet.css").unwrap();
    builder.into_stylesheet().write(&mut css_out).unwrap();

    let end = Instant::now();
    let delta = end.duration_since(start).as_secs_f64();
    println!("Built in {}s", delta);
}
