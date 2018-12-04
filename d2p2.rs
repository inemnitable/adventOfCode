mod input;
mod d2libs;

fn main() {
  let the_input = input::get_the_input();
  let parsed_input = d2libs::parse_the_input(&the_input);

  for i in 0..parsed_input.len() {
    for j in i..parsed_input.len() {
      if is_similar(parsed_input[i], parsed_input[j]) {
        print_same_chs(parsed_input[i], parsed_input[j]);
      }
    }
  }
}

fn is_similar(s1: &str, s2: &str) -> bool {
  if s1.len() != s2.len() { 
    return false;
  }

  let mut differing = 0;
  for i in 0..s1.len() {
    if s1.chars().nth(i).expect("s1i nth") != s2.chars().nth(i).expect("s2i nth") {
      differing += 1;
    }
  }
  return differing == 1;
}

fn print_same_chs<'a>(s1: &'a str, s2: &'a str) {
  for i in 0..s1.len() {
    let c1 = s1.chars().nth(i).unwrap();
    let c2 = s2.chars().nth(i).unwrap();
    if c1 == c2 {
      print!("{}", c1);
    }
  }
}