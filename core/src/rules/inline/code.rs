use crate::{helpers::find_closing, nodes::inline::Inline, rules::InlineRule};

pub struct CodeRule;
impl InlineRule for CodeRule {
  fn matches(&self, chars: &[char], index: usize) -> bool {
    chars[index] == '`'
  }

  fn parse(
    &self,
    chars: &[char],
    mut index: usize,
    _parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::inline::Inline, usize)> {
    let mut backtick_count = 0;
    while index < chars.len() && chars[index] == '`' {
      backtick_count += 1;
      index += 1;
    }

    let closing_pattern = "`".repeat(backtick_count);

    if let Some(end) = find_closing(chars, index, &closing_pattern) {
      let content: String = chars[index..end].iter().collect();

      let trimmed_content = if content.starts_with(' ') && content.ends_with(' ') && content.len() > 2 {
        content[1..content.len() - 1].to_string()
      } else if content.starts_with(' ') && content.len() > 1 {
        content[1..].to_string()
      } else if content.ends_with(' ') && content.len() > 1 {
        content[..content.len() - 1].to_string()
      } else {
        content
      };

      Some((Inline::Code(trimmed_content), end + backtick_count))
    } else {
      None
    }
  }
}
