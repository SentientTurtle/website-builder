use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};

#[allow(dead_code)]
#[derive(Debug, Eq, PartialEq)]
pub enum Language {
    English,
    CUSTOM {
        tag: String
    }
}

impl Language {
    pub fn as_rfc5646_tag(&self) -> &str {
        match self {
            Language::English => "en",
            Language::CUSTOM { tag } => tag
        }
    }
}

pub trait VecExt<T> {
    fn vec_map<U, F: FnMut(T) -> U>(self, f: F) -> Vec<U>;

    fn push_chain(self, value: T) -> Self;

    fn extend_chain<I: IntoIterator<Item = T>>(self, iter: I) -> Self;
}

impl<T> VecExt<T> for Vec<T> {
    fn vec_map<U, F: FnMut(T) -> U>(self, f: F) -> Vec<U> {
        self.into_iter()
            .map(f)
            .collect()
    }

    fn push_chain(mut self, value: T) -> Self {
        self.push(value);
        self
    }

    fn extend_chain<I: IntoIterator<Item = T>>(mut self, iter: I) -> Self {
        self.extend(iter);
        self
    }
}

pub trait DisplayExt {
    fn display_string(self) -> String;
}

impl<T: Display> DisplayExt for T {
    fn display_string(self) -> String {
        format!("{}", self)
    }
}


static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn next_unique_id() -> u64 {
    ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}