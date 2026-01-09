use crate::{nodes::block::Block, rules::BlockRule};

pub struct BlockQuoteRule;
impl BlockRule for BlockQuoteRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool {
    lines[index].starts_with('>')
  }

  fn parse(
    &self,
    lines: &[&str],
    mut index: usize,
    parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::block::Block, usize)> {
    let mut quote_lines = Vec::new();
    while index < lines.len() && lines[index].trim_start().starts_with('>') {
      let content = lines[index].trim_start().trim_start_matches('>').trim();
      quote_lines.push(content);
      index += 1;
    }
    let quote_text = quote_lines.join("\n");
    Some((Block::BlockQuote(parser.parse(&quote_text)), index))
  }
}
