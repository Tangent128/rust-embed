#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
extern crate walkdir;

#[allow(unused_imports)]
#[macro_use]
extern crate rust_embed_impl;
pub use rust_embed_impl::*;

#[doc(hidden)]
#[cfg(all(debug_assertions, not(feature = "debug-embed")))]
pub mod utils;

pub trait RustEmbed {
  fn get(&self, file_path: &str) -> Option<std::borrow::Cow<'static, [u8]>>;
  fn iter(&self) -> Filenames;
}

pub enum Filenames {
  Embedded(std::slice::Iter<'static, &'static str>),
  Dynamic(Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>),
}

impl Iterator for Filenames {
  type Item = std::borrow::Cow<'static, str>;
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      Filenames::Embedded(names) => names.next().map(|x| std::borrow::Cow::from(*x)),
      Filenames::Dynamic(boxed) => boxed.next(),
    }
  }
}
