use crate::{nodes::block::Block, rules::BlockRule};

pub struct HeadingRule;
impl BlockRule for HeadingRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool {
    lines[index].starts_with('#')
  }

  fn parse(
    &self,
    lines: &[&str],
    index: usize,
    parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::block::Block, usize)> {
    let line = lines[index];
    let level = line.chars().take_while(|&c| c == '#').count();
    if level > 6 {
      return None;
    }
    let text = line[level..].trim();
    Some((Block::Heading(level as u8, parser.parse_inline(text)), index + 1))
  }
}
