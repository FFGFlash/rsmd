pub fn find_closing(chars: &[char], start: usize, pattern: &str) -> Option<usize> {
  let pat_chars: Vec<char> = pattern.chars().collect();
  let pat_len = pat_chars.len();

  for i in start..chars.len() {
    if i + pat_len <= chars.len() {
      if &chars[i..i + pat_len] == pat_chars.as_slice() {
        return Some(i);
      }
    }
  }
  None
}

pub fn find_char(chars: &[char], start: usize, target: char) -> Option<usize> {
  chars[start..]
    .iter()
    .position(|&c| c == target)
    .map(|p| p + start)
}
