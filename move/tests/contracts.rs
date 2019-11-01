use functional_tests::{checker::check, evaluator::eval, utils::parse_input};
use sprint_move::generate;
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
fn generated_zero() {
    let suite = Path::new("tests/suites/zero.test.mvir");
    let contract = contract("zero").unwrap();

    test(generate(&contract), suite);
}

#[test]
fn generated_one() {
    let suite = Path::new("tests/suites/one.test.mvir");
    let contract = contract("one").unwrap();

    test(generate(&contract), suite);
}

#[test]
fn generated_one_and_one() {
    let suite = Path::new("tests/suites/one_and_one.test.mvir");
    let contract = contract("one and one").unwrap();

    test(generate(&contract), suite);
}
