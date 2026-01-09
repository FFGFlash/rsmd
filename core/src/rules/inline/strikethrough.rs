use crate::{helpers::find_closing, nodes::inline::Inline, rules::InlineRule};

pub struct StrikethroughRule;
impl InlineRule for StrikethroughRule {
  fn matches(&self, chars: &[char], index: usize) -> bool {
    index + 1 < chars.len() && chars[index] == '~' && chars[index + 1] == '~'
  }

  fn parse(
    &self,
    chars: &[char],
    index: usize,
    parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::inline::Inline, usize)> {
    if let Some(end) = find_closing(&chars, index + 2, "~~") {
      let content: String = chars[index + 2..end].iter().collect();
      Some((Inline::Strikethrough(parser.parse_inline(&content)), end + 2))
    } else {
      None
    }
  }
}
