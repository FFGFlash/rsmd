use crate::{nodes::block::Block, rules::BlockRule};

pub struct CodeBlockRule;
impl BlockRule for CodeBlockRule {
  fn matches(&self, lines: &[&str], index: usize) -> bool {
    lines[index].starts_with("```")
  }

  fn parse(
    &self,
    lines: &[&str],
    mut index: usize,
    _parser: &crate::MarkdownParser,
  ) -> Option<(crate::nodes::block::Block, usize)> {
    let lang = lines[index][3..].trim().to_string();
    index += 1;
    let mut code_lines = Vec::new();
    while index < lines.len() && !lines[index].starts_with("```") {
      code_lines.push(lines[index]);
      index += 1;
    }
    index += 1; // skip closing ```
    Some((Block::CodeBlock(lang, code_lines.join("\n")), index))
  }
}
