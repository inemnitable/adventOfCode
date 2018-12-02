mod input;
mod d1libs;

fn main() {
  let the_input = input::get_the_input();

  let the_deltas: Vec<isize> = d1libs::parse_the_deltas(the_input);
  let sum: isize = the_deltas.iter().sum();
  println!("{}", sum);
}

