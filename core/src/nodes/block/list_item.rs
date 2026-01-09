use std::fmt;

use crate::{
  MarkdownParser,
  nodes::{block::BlockList, inline::InlineList},
};

#[derive(Debug, Clone)]
pub struct ListItem {
  pub content: InlineList,
  pub nested_blocks: BlockList,
}

impl fmt::Display for ListItem {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "<li>{}{}</li>", self.content, self.nested_blocks)
  }
}

pub fn parse_list_items(
  lines: &[&str],
  index: &mut usize,
  parser: &MarkdownParser,
  is_unordered: bool,
) -> Vec<ListItem> {
  let mut items: Vec<ListItem> = Vec::new();
  let base_indent = get_indent(lines[*index]);

  while *index < lines.len() {
    let line = lines[*index];
    let indent = get_indent(line);
    let trimmed = line.trim_start();

    if trimmed.is_empty() {
      *index += 1;

      if *index < lines.len() {
        let next_indent = get_indent(lines[*index]);
        let next_trimmed = lines[*index].trim_start();
        let next_is_list = if is_unordered {
          next_trimmed.starts_with("- ") || next_trimmed.starts_with("* ")
        } else {
          next_trimmed.chars().take_while(|c| c.is_numeric()).count() > 0 && next_trimmed.contains(". ")
        };

        if next_is_list && next_indent >= base_indent {
          continue;
        }
      }
      *index -= 1;
      break;
    }

    let is_item = if is_unordered {
      (trimmed.starts_with("- ") || trimmed.starts_with("* ")) && indent == base_indent
    } else {
      indent == base_indent && {
        if let Some(pos) = trimmed.find(". ") {
          trimmed[..pos].chars().all(|c| c.is_numeric())
        } else {
          false
        }
      }
    };

    if !is_item {
      if indent > base_indent {
        if let Some(last_item) = items.last_mut() {
          let mut nested_lines = Vec::new();

          while *index < lines.len() {
            let current_indent = get_indent(lines[*index]);
            if current_indent > base_indent || lines[*index].trim().is_empty() {
              nested_lines.push(lines[*index]);
              *index += 1;
            } else {
              break;
            }
          }

          if !nested_lines.is_empty() {
            let min_indent = nested_lines
              .iter()
              .filter(|l| !l.trim().is_empty())
              .map(|l| get_indent(l))
              .min()
              .unwrap_or(0);

            let dedented: Vec<String> = nested_lines
              .iter()
              .map(|l| {
                if l.trim().is_empty() {
                  String::new()
                } else {
                  l.chars().skip(min_indent).collect()
                }
              })
              .collect();

            let nested_text = dedented.join("\n");
            last_item.nested_blocks = parser.parse(&nested_text);
          }
          continue;
        }
      }
      break;
    }

    let content = if is_unordered {
      trimmed[2..].trim()
    } else {
      if let Some(pos) = trimmed.find(". ") {
        trimmed[pos + 2..].trim()
      } else {
        break;
      }
    };

    items.push(ListItem {
      content: parser.parse_inline(content),
      nested_blocks: BlockList(Vec::new()),
    });
    *index += 1;
  }

  items
}

fn get_indent(line: &str) -> usize {
  line.chars().take_while(|c| c.is_whitespace()).count()
}
