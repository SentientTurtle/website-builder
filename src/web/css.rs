use std::collections::HashSet;
use std::io::Write;
use indexmap::IndexSet;

pub type CSSRule = (CSSQuery, &'static str, Box<[&'static str]>);

pub type CSSCallback = fn() -> CSSRule;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum CSSQuery {
    None,
    Media(&'static str),
}

pub struct CSSBuilder {
    imports: IndexSet<String>,
    rules: IndexSet<CSSCallback>
}

impl CSSBuilder {
    pub fn new() -> CSSBuilder {
        CSSBuilder {
            imports: IndexSet::new(),
            rules: IndexSet::new()
        }
    }

    pub fn import<S: ToString>(&mut self, source: S) {
        self.imports.insert(source.to_string());
    }

    pub fn register(&mut self, generator: CSSCallback) {
        self.rules.insert(generator);
    }

    fn escape_identifier(identifier: &str) -> String {
        use std::fmt::Write;
        let mut string = String::with_capacity(identifier.len());

        let mut first_char_is_hyphen = false;

        for (index, char) in identifier.chars().enumerate() {
            if char == '\u{0}' {
                string.push('\u{FFFD}');
            } else if ('\u{1}'..='\u{1F}').contains(&char)
                || (index == 0 && ('\u{30}'..='\u{39}').contains(&char))
                || (index == 1 && ('\u{30}'..='\u{39}').contains(&char) && first_char_is_hyphen)
            {
                write!(string, "\\{:x} ", char as u32).unwrap();
            } else if char == '\u{2D}' && identifier.len() == 1 {
                string.push_str("\\-");
            } else if char == '\u{2D}' {
                first_char_is_hyphen = true;
                string.push(char);
            } else if char >= '\u{80}'
                || char == '\u{2D}'
                || char == '\u{5F}'
                || ('\u{30}'..='\u{39}').contains(&char)
                || ('\u{41}'..='\u{5A}').contains(&char)
                || ('\u{61}'..='\u{7A}').contains(&char)
            {
                string.push(char);
            } else {
                string.push('\\');
                string.push(char);
            }
        }

        string
    }

    pub fn write<W: Write>(self, out: &mut W) -> std::io::Result<()> {
        let mut seen_identifiers = HashSet::new();

        for import in &self.imports {
            writeln!(out, "@import {};", import)?;
        }

        if self.imports.len() > 0 {
            writeln!(out)?;
        }

        for (query, identifier, contents) in self.rules.iter().map(|callback| callback()) {
            if !seen_identifiers.insert((query, identifier)) {
                panic!("duplicate style declaration for {:?} {}", query, identifier)
            }

            let mut indent: usize = match query {
                CSSQuery::None => 0,
                CSSQuery::Media(media) => {
                    writeln!(out, "@media {} {{", media)?;
                    1
                }
            };
            writeln!(out, "{:indent$}{} {{", "", identifier, indent = (indent * 4))?;
            indent += 1;
            for property in contents.into_iter() {
                writeln!(out, "{:indent$}{};", "", *property, indent = (indent) * 4)?;
            }
            while indent > 0 {
                indent -= 1;
                writeln!(out, "{:indent$}}}", "", indent = (indent * 4))?;
            }
            writeln!(out)?;
        }

        Ok(())
    }
}


macro_rules! css {
    ($document:expr, $(@Media:$media:expr,)? Root, [$($declaration:expr),+]) => {
        let document: &mut $crate::web::css::CSSBuilder = $document;
        document.register(|| (
            {
                #[allow(unused_variables)]
                let query = $crate::web::css::CSSQuery::None;
                $(let query = $crate::web::css::CSSQuery::Media($media);)?
                query
            },
            ":root",
            Box::new([
                $($declaration),+
            ])
        ));
    };
    ($document:expr, $(@Media:$media:expr,)? Tag:$identifier:expr, [$($declaration:expr),+]) => {
        let document: &mut $crate::web::css::CSSBuilder = $document;
        document.register(|| (
            {
                #[allow(unused_variables)]
                let query = $crate::web::css::CSSQuery::None;
                $(let query = $crate::web::css::CSSQuery::Media($media);)?
                query
            },
            $identifier,
            Box::new([
                $($declaration),+
            ])
        ));
    };
    ($document:expr, $(@Media:$media:expr,)? Class:$identifier:expr, [$($declaration:expr),+]) => {
        let document: &mut $crate::web::css::CSSBuilder = $document;
        document.register(|| (
            {
                #[allow(unused_variables)]
                let query = $crate::web::css::CSSQuery::None;
                $(let query = $crate::web::css::CSSQuery::Media($media);)?
                query
            },
            concat!(".", $identifier),
            Box::new([
                $($declaration),+
            ])
        ));
    };
}
