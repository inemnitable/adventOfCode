mod input;
mod d2libs;

use std::collections::HashMap;

fn main() {
  let the_input = input::get_the_input();
  let mut parsed_input = d2libs::parse_the_input(&the_input);
  let the_twos = count_the_nles(&mut parsed_input, 2);
  let the_threes = count_the_nles(&mut parsed_input, 3);

  println!("{}", the_twos * the_threes);
}

fn count_the_nles(ss: &mut Vec<&str>, n: usize) -> usize {
  ss.iter().fold(0, |acc, s| if has_an_nle(s, n) {acc + 1} else { acc })
}

fn has_an_nle(s: &str, n: usize) -> bool {
  let mut counts: HashMap<char, usize> = HashMap::new();
  for c in s.chars() {
    let mut old_value = match counts.get(&c) {
        None => 0usize,
        Some(i) => *i,
      };
    
    counts.insert(c, old_value + 1);
  }
  return counts.values().any(|el| *el == n);
}

