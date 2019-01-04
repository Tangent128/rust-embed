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
  fn iter(&self) -> Box<dyn Iterator<Item = std::borrow::Cow<'static, str>>>;
}
