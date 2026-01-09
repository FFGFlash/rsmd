use crate::{
  nodes::block::{Block, list_item::parse_list_items},
  rules::BlockRule,
};

pub struct UnorderedListRule;
impl BlockRule for UnorderedListRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool {
    let trimmed = lines[index].trim_start();
    trimmed.starts_with("- ") || trimmed.starts_with("* ")
  }

  fn parse(
    &self,
    lines: &[&str],
    mut index: usize,
    parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::block::Block, usize)> {
    let items = parse_list_items(lines, &mut index, parser, true);
    Some((Block::UnorderedList(items), index))
  }
}
