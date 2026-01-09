use crate::{
  MarkdownParser,
  nodes::{block::Block, inline::Inline},
};

pub mod block;
pub mod inline;

pub trait BlockRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool;
  fn parse(&self, lines: &[&str], index: usize, parser: &MarkdownParser) -> Option<(Block, usize)>;
  fn on_emit(&self, emitted: &Block, output: &mut Vec<Block>) {
    let _ = (emitted, output);
  }
}

pub trait InlineRule {
  fn matches(&self, chars: &[char], index: usize) -> bool;
  fn parse(&self, chars: &[char], index: usize, parser: &MarkdownParser) -> Option<(Inline, usize)>;
  fn on_emit(&self, emitted: &Inline, output: &mut Vec<Inline>) {
    let _ = (emitted, output);
  }
}
