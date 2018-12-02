mod input;
mod d1libs;

use std::collections::HashSet;

fn main() {
  let the_input = input::get_the_input();
  let the_deltas: Vec<isize> = d1libs::parse_the_deltas(the_input);

  let mut seen: HashSet<isize> = HashSet::new();
  let mut the_total: isize = 0;
  let mut pos = 0;

  loop {
    the_total += the_deltas[pos];
    if seen.contains(&the_total) {
      println!("{}", the_total);
      break;
    } else {
      seen.insert(*&the_total);
    }
    if pos < the_deltas.len() - 1 {
      pos += 1;
    } else {
      pos = 0;
    }
  }
}