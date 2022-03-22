#[macro_use]
extern crate napi_derive;

use pulldown_cmark::{Options, Parser, html};

#[napi]
pub fn transform(content: String) -> String {
  let mut options = Options::empty();
  options.insert(Options::ENABLE_STRIKETHROUGH);
  let parser = Parser::new_ext(&content, options);
  let mut html_output: String = String::with_capacity(&content.len() * 3 / 2);
  html::push_html(&mut html_output, parser);

  html_output
}
