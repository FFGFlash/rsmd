use crate::{helpers::find_closing, nodes::inline::Inline, rules::InlineRule};

pub struct ItalicRule {
  marker: String,
}

impl ItalicRule {
  pub fn new(marker: &str) -> Self {
    Self {
      marker: marker.to_string(),
    }
  }
}

impl InlineRule for ItalicRule {
  fn matches(&self, chars: &[char], index: usize) -> bool {
    let needle_len = self.marker.chars().count();

    let hay = match chars.get(index..index + needle_len) {
      Some(slice) => slice,
      None => return false,
    };

    hay.iter().zip(self.marker.chars()).all(|(a, b)| *a == b)
  }

  fn parse(
    &self,
    chars: &[char],
    index: usize,
    parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::inline::Inline, usize)> {
    let marker_chars: Vec<char> = self.marker.chars().collect();
    if let Some(end) = find_closing(&chars, index + marker_chars.len(), &self.marker) {
      let content: String = chars[index + marker_chars.len()..end].iter().collect();
      let new_index = end + marker_chars.len();
      Some((Inline::Bold(parser.parse_inline(&content)), new_index))
    } else {
      None
    }
  }
}
