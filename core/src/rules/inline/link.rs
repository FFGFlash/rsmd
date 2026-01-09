use crate::{helpers::find_char, nodes::inline::Inline, rules::InlineRule};

pub struct LinkRule;
impl InlineRule for LinkRule {
  fn matches(&self, chars: &[char], index: usize) -> bool {
    chars[index] == '['
  }

  fn parse(
    &self,
    chars: &[char],
    index: usize,
    _parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::inline::Inline, usize)> {
    if let Some(text_end) = find_char(chars, index + 1, ']') {
      if text_end + 1 < chars.len() && chars[text_end + 1] == '(' {
        if let Some(url_end) = find_char(chars, text_end + 2, ')') {
          let text: String = chars[index + 1..text_end].iter().collect();
          let url: String = chars[text_end + 2..url_end].iter().collect();
          return Some((Inline::Link(text, url), url_end + 1));
        }
      }
    }
    None
  }
}
