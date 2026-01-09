use std::{fmt, ops::Deref};

pub trait CustomInlineNode: fmt::Display + fmt::Debug + CloneInlineNode {
  fn as_any(&self) -> &dyn std::any::Any;
}

pub trait CloneInlineNode {
  fn clone_box(&self) -> Box<dyn CustomInlineNode>;
}

impl<T> CloneInlineNode for T
where
  T: 'static + CustomInlineNode + Clone,
{
  fn clone_box(&self) -> Box<dyn CustomInlineNode> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn CustomInlineNode> {
  fn clone(&self) -> Self {
    self.clone_box()
  }
}

#[derive(Debug, Clone)]
pub struct InlineList(pub Vec<Inline>);

impl fmt::Display for InlineList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for inline in &self.0 {
      write!(f, "{}", inline)?;
    }
    Ok(())
  }
}

impl IntoIterator for InlineList {
  type Item = Inline;
  type IntoIter = std::vec::IntoIter<Inline>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a InlineList {
  type Item = &'a Inline;
  type IntoIter = std::slice::Iter<'a, Inline>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for InlineList {
  type Target = [Inline];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Clone)]
pub enum Inline {
  Text(String),
  Bold(InlineList),
  Italic(InlineList),
  Strikethrough(InlineList),
  Code(String),
  Link(String, String),
  Break,
  Custom(Box<dyn CustomInlineNode>),
}

impl fmt::Display for Inline {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Text(text) => write!(f, "{}", text),
      Self::Bold(content) => write!(f, "<strong>{}</strong>", content),
      Self::Italic(content) => write!(f, "<em>{}</em>", content),
      Self::Strikethrough(content) => write!(f, "<del>{}</del>", content),
      Self::Code(code) => write!(f, "<code>{}</code>", code),
      Self::Link(text, url) => write!(f, "<a href=\"{}\">{}</a>", url, text),
      Self::Break => write!(f, "<br />"),
      Self::Custom(custom) => write!(f, "{}", custom),
    }
  }
}
