use crate::{nodes::block::Block, rules::BlockRule};

pub struct HorizontalRuleRule;
impl BlockRule for HorizontalRuleRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool {
    lines[index].starts_with("---") || lines[index].starts_with("***")
  }

  fn parse(
    &self,
    _lines: &[&str],
    index: usize,
    _parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::block::Block, usize)> {
    Some((Block::HorizontalRule, index + 1))
  }
}
