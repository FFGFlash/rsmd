use crate::{
  nodes::{
    block::Block,
    inline::{Inline, InlineList},
  },
  rules::{BlockRule, InlineRule},
};

pub trait EmitHook {
  fn on_inline_emit(&self, emitted: &mut Inline, output: &mut Vec<Inline>) {
    let _ = (emitted, output);
  }
  fn on_block_emit(&self, emitted: &mut Block, output: &mut Vec<Block>) {
    let _ = (emitted, output);
  }
}

pub enum EmitSource<'a> {
  InlineRule(&'a dyn InlineRule),
  BlockRule(&'a dyn BlockRule),
  Parser,
}

pub struct ParagraphNormalizeHook;
impl EmitHook for ParagraphNormalizeHook {
  fn on_block_emit(&self, emitted: &mut Block, _output: &mut Vec<Block>) {
    if let Block::Paragraph(inlines) = emitted {
      normalize_paragraph_inlines(inlines);
    }
  }
}

fn normalize_paragraph_inlines(inlines: &mut InlineList) {
  for inline in &mut inlines.0 {
    if let Inline::Text(text) = inline {
      *text = text.replace('\n', " ")
    }
  }
}
