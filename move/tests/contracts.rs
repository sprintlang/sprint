use functional_tests::{checker::check, evaluator::eval, utils::parse_input};
use sprint_move::generator::Generator;
use sprint_parser::parser::contract;
use std::{
  fmt::Display,
  fs::{self, File},
  io::{prelude::*, Read},
  path::Path,
};

fn test(module: impl Display, suite: &Path) {
  let mut input = String::new();
  let file_name = suite.file_name().unwrap().to_str().unwrap();

  input.push_str("//! account: alice, 1000000\n");
  input.push_str("//! account: bob, 1000000\n");
  input.push_str("//! account: chris, 1000000\n\n");

  input.push_str("//! new-transaction\n");
  input.push_str("//! sender: alice\n");

  input.push_str(&format!("{}\n", module));

  let mut suite = File::open(suite).unwrap();
  suite.read_to_string(&mut input).unwrap();

  // TODO: Only create file if flag is set.
  fs::create_dir_all("tests/generated").unwrap();
  let generated_file = format!("tests/generated/{}", &file_name);
  let mut file = File::create(&generated_file).unwrap();
  file.write_all(input.as_bytes()).unwrap();

  let (config, directives, transactions) = parse_input(&input).unwrap();
  let log = eval(&config, &transactions).unwrap();

  if let Err(err) = check(&log, &directives) {
    println!("{}", log);
    panic!(err);
  }
}

#[test]
fn generated_zero_is_valid_move() {
  let suite = Path::new("tests/move_tests/none.test.mvir");

  let mut generator = Generator::new("Zero");
  let state = contract("zero").unwrap();
  generator.generate(&state);

  test(generator.contract(), suite);
}

#[test]
fn generated_zero_has_correct_behavior() {
  let suite = Path::new("tests/move_tests/zero.test.mvir");

  let mut generator = Generator::new("Zero");
  let state = contract("zero").unwrap();
  generator.generate(&state);

  test(generator.contract(), suite);
}

#[test]
fn generated_one_is_valid_move() {
  let suite = Path::new("tests/move_tests/none.test.mvir");

  let mut generator = Generator::new("One");
  let state = contract("one").unwrap();
  generator.generate(&state);

  test(generator.contract(), suite);
}

#[test]
fn generated_one_has_correct_behavior() {
  let suite = Path::new("tests/move_tests/one.test.mvir");

  let mut generator = Generator::new("One");
  let state = contract("one").unwrap();
  generator.generate(&state);

  test(generator.contract(), suite);
}
