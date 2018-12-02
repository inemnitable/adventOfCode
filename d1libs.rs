pub fn parse_the_deltas<'a>(input: String) -> Vec<isize> {
  let mut the_deltas : Vec<isize> = Vec::new();
  for mut line in input.lines() {
    if &line[..0] == "+" {
      line = &line[1..];
    }
    the_deltas.push(line.parse::<isize>().unwrap())
  }
  return the_deltas;
}