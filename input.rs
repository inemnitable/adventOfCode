use std::env;
use std::fs;
use std::ffi::OsString;

pub fn get_the_input() -> String {
  let args: Vec<OsString> = env::args_os().collect();
  let filename = &args[1];
  let input = fs::read_to_string(filename).unwrap();

  return input;
}
