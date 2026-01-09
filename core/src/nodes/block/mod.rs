use std::{fmt, ops::Deref};
pub mod list_item;

use crate::nodes::{block::list_item::ListItem, inline::InlineList};

pub trait CustomBlockNode: fmt::Display + fmt::Debug + CloneBlockNode {
  fn as_any(&self) -> &dyn std::any::Any;
}

pub trait CloneBlockNode {
  fn clone_box(&self) -> Box<dyn CustomBlockNode>;
}

impl<T> CloneBlockNode for T
where
  T: 'static + CustomBlockNode + Clone,
{
  fn clone_box(&self) -> Box<dyn CustomBlockNode> {
    Box::new(self.clone())
  }
}

impl Clone for Box<dyn CustomBlockNode> {
  fn clone(&self) -> Self {
    self.clone_box()
  }
}

#[derive(Debug, Clone)]
pub struct BlockList(pub Vec<Block>);

impl fmt::Display for BlockList {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for block in &self.0 {
      write!(f, "{}", block)?;
    }
    Ok(())
  }
}

impl IntoIterator for BlockList {
  type Item = Block;
  type IntoIter = std::vec::IntoIter<Block>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.into_iter()
  }
}

impl<'a> IntoIterator for &'a BlockList {
  type Item = &'a Block;
  type IntoIter = std::slice::Iter<'a, Block>;

  fn into_iter(self) -> Self::IntoIter {
    self.0.iter()
  }
}

impl Deref for BlockList {
  type Target = [Block];

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

#[derive(Debug, Clone)]
pub enum Block {
  Heading(u8, InlineList),
  Paragraph(InlineList),
  CodeBlock(String, String),
  BlockQuote(BlockList),
  UnorderedList(Vec<ListItem>),
  OrderedList(Vec<ListItem>),
  HorizontalRule,
  Custom(Box<dyn CustomBlockNode>),
}

impl fmt::Display for Block {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Heading(level, content) => write!(f, "<h{level}>{content}</h{level}>"),
      Self::Paragraph(content) => write!(f, "<p>{}</p>", content),
      Self::CodeBlock(lang, code) => write!(f, "<p><code class=\"language-{}\">{}</code></pre>", lang, code),
      Self::BlockQuote(blocks) => write!(f, "<blockquote>{}</blockquote>", blocks),
      Self::UnorderedList(items) => {
        write!(f, "<ul>")?;
        for item in items {
          write!(f, "{}", item)?;
        }
        write!(f, "</ul>")
      }
      Self::OrderedList(items) => {
        write!(f, "<ol>")?;
        for item in items {
          write!(f, "{}", item)?;
        }
        write!(f, "</ol>")
      }
      Self::HorizontalRule => write!(f, "<hr />"),
      Self::Custom(custom) => write!(f, "{}", custom),
    }
  }
}
