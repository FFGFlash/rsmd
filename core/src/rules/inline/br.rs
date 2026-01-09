use crate::{MarkdownParser, nodes::inline::Inline, rules::InlineRule};

pub struct BreakRule;
impl InlineRule for BreakRule {
  fn matches(&self, chars: &[char], index: usize) -> bool {
    if chars.get(index) != Some(&'\n') {
      return false;
    }

    if index >= 1 && chars[index - 1] == '\\' {
      return true;
    }

    if index >= 2 && chars[index - 1] == ' ' && chars[index - 2] == ' ' {
      return true;
    }

    false
  }

  fn parse(&self, _chars: &[char], index: usize, _parser: &MarkdownParser) -> Option<(Inline, usize)> {
    Some((Inline::Break, index + 1))
  }

  fn on_emit(&self, _emitted: &Inline, output: &mut Vec<Inline>) {
    if let Some(Inline::Text(text)) = output.last_mut() {
      while text.ends_with(' ') {
        text.pop();
      }
      if text.ends_with('\\') {
        text.pop();
      }
    }
  }
}
