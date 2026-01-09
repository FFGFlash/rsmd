use crate::{
  hooks::{EmitHook, EmitSource, ParagraphNormalizeHook},
  nodes::{
    block::{Block, BlockList},
    inline::{Inline, InlineList},
  },
  rules::{
    BlockRule, InlineRule,
    block::{
      BlockQuoteRule, CodeBlockRule, HeadingRule, HorizontalRuleRule, OrderedListRule, UnorderedListRule,
    },
    inline::{BoldRule, BreakRule, CodeRule, ItalicRule, LinkRule, StrikethroughRule},
  },
};

pub mod helpers;
pub mod hooks;
pub mod nodes;
pub mod rules;

pub struct MarkdownParser {
  inline_rules: Vec<Box<dyn InlineRule>>,
  block_rules: Vec<Box<dyn BlockRule>>,
  emit_hooks: Vec<Box<dyn EmitHook>>,
}

impl MarkdownParser {
  pub fn new() -> Self {
    Self {
      inline_rules: Vec::new(),
      block_rules: Vec::new(),
      emit_hooks: Vec::new(),
    }
  }

  pub fn with_defaults() -> Self {
    let mut parser = Self {
      inline_rules: Vec::new(),
      block_rules: Vec::new(),
      emit_hooks: Vec::new(),
    };

    parser.register_defaults();

    parser
  }

  pub fn register_defaults(&mut self) {
    self.register_default_rules();
    self.register_default_hooks();
  }

  pub fn register_default_rules(&mut self) {
    self.add_block_rule(Box::new(HorizontalRuleRule));
    self.add_block_rule(Box::new(HeadingRule));
    self.add_block_rule(Box::new(CodeBlockRule));
    self.add_block_rule(Box::new(BlockQuoteRule));
    self.add_block_rule(Box::new(UnorderedListRule));
    self.add_block_rule(Box::new(OrderedListRule));

    self.add_inline_rule(Box::new(BoldRule::new("**")));
    self.add_inline_rule(Box::new(BoldRule::new("__")));
    self.add_inline_rule(Box::new(ItalicRule::new("*")));
    self.add_inline_rule(Box::new(ItalicRule::new("_")));
    self.add_inline_rule(Box::new(StrikethroughRule));
    self.add_inline_rule(Box::new(CodeRule));
    self.add_inline_rule(Box::new(LinkRule));
    self.add_inline_rule(Box::new(BreakRule));
  }

  pub fn register_default_hooks(&mut self) {
    self.add_emit_hook(Box::new(ParagraphNormalizeHook));
  }

  pub fn add_block_rule(&mut self, rule: Box<dyn BlockRule>) {
    self.block_rules.push(rule);
  }

  pub fn add_inline_rule(&mut self, rule: Box<dyn InlineRule>) {
    self.inline_rules.push(rule);
  }

  pub fn add_emit_hook(&mut self, hook: Box<dyn EmitHook>) {
    self.emit_hooks.push(hook);
  }

  pub fn parse_to_html(&self, input: &str) -> String {
    self.parse(input).to_string()
  }

  pub fn parse(&self, input: &str) -> BlockList {
    let mut result = Vec::new();
    let lines: Vec<&str> = input.lines().collect();
    let mut i = 0;

    'outer: while i < lines.len() {
      let line = lines[i].trim_end();

      if line.is_empty() {
        i += 1;
        continue;
      }

      for rule in &self.block_rules {
        if rule.matches(&lines, i) {
          if let Some((block, new_index)) = rule.parse(&lines, i, self) {
            self.emit_block(&mut result, block, EmitSource::BlockRule(rule.as_ref()));
            i = new_index;
            continue 'outer;
          }
        }
      }

      let mut para_lines = Vec::new();
      'inner: while i < lines.len() {
        let l = lines[i];
        if l.is_empty() {
          break;
        }

        for rule in &self.block_rules {
          if rule.matches(&lines, i) {
            break 'inner;
          }
        }

        para_lines.push(l);
        i += 1;
      }
      if !para_lines.is_empty() {
        let text = para_lines.join("\n");
        self.emit_block(
          &mut result,
          Block::Paragraph(self.parse_inline(&text)),
          EmitSource::Parser,
        );
      } else {
        //? I think we only get to this case if a rule fails to parse
        i += 1;
      }
    }

    BlockList(result)
  }

  pub fn emit_block(&self, output: &mut Vec<Block>, mut block: Block, source: EmitSource) {
    if let EmitSource::BlockRule(rule) = source {
      rule.on_emit(&block, output);
    }
    for hook in &self.emit_hooks {
      hook.on_block_emit(&mut block, output);
    }
    output.push(block);
  }

  pub fn parse_inline(&self, input: &str) -> InlineList {
    let mut result = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    'main: while i < chars.len() {
      for rule in &self.inline_rules {
        if rule.matches(&chars, i) {
          if let Some((inline, new_index)) = rule.parse(&chars, i, self) {
            self.emit_inline(&mut result, inline, EmitSource::InlineRule(rule.as_ref()));
            i = new_index;
            continue 'main;
          }
        }
      }

      if let Some(Inline::Text(text)) = result.last_mut() {
        text.push(chars[i]);
      } else {
        self.emit_inline(
          &mut result,
          Inline::Text(chars[i].to_string()),
          EmitSource::Parser,
        );
      }
      i += 1;
    }

    InlineList(result)
  }

  pub fn emit_inline(&self, output: &mut Vec<Inline>, mut inline: Inline, source: EmitSource) {
    if let EmitSource::InlineRule(rule) = source {
      rule.on_emit(&inline, output);
    }
    for hook in &self.emit_hooks {
      hook.on_inline_emit(&mut inline, output);
    }
    output.push(inline);
  }
}
