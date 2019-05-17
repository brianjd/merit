extern crate scraper;

use super::get_color;
use lazy_static::*;
use scraper::{Html, Selector};
use std::{borrow::Cow, collections::HashMap, str};

static DEFAULT_COLOUR: &str = "fff";

lazy_static! {
  static ref SYMBOLS: HashMap<String, String> = {
    let mut symbols: HashMap<String, String> = HashMap::new();
    let sources = [include_str!("../resx/icons/brands.svg")];
    for src in sources.iter() {
      let doc = Html::parse_fragment(src);
      let selector = Selector::parse("symbol").unwrap();
      for el in doc.select(&selector) {
        let id = el.value().attr("id").unwrap().to_owned();
        let sym = el.html();
        symbols.insert(id, sym);
      }
    }
    symbols
  };
}

pub fn icon_exists(icon_name: &str) -> bool {
  SYMBOLS.contains_key(icon_name)
}

#[derive(Debug, PartialEq, Eq)]
pub struct Icon<'a> {
  pub name: &'a str,
  pub size: u32,
  pub color: Cow<'a, str>,
  pub symbol: &'a str,
}
impl<'a> Icon<'a> {
  pub fn new(name: &'a str) -> Option<Self> {
    if let Some(icon) = SYMBOLS.get(name) {
      let icon = Icon {
        color: get_color(DEFAULT_COLOUR).unwrap().into(),
        name: name,
        size: 13,
        symbol: icon.as_str(),
      };
      Some(icon)
    } else {
      None
    }
  }
  pub fn color<S>(&mut self, color: S) -> &mut Self
  where
    S: Into<Cow<'a, str>>,
  {
    let color = color.into();
    if let Some(c) = get_color(&color) {
      self.color = c.into();
    }
    self
  }
  pub fn size(&mut self, size: u32) -> &mut Self {
    self.size = size;
    self
  }
}

#[cfg(test)]
mod tests {
  use super::Icon;

  #[test]
  fn get_icon_symbol() {
    let icon = Icon::new("bluetooth-b");
    assert!(icon.is_some());
    assert!(icon.unwrap().symbol.len() > 0);
  }

}