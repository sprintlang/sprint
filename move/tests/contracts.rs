use functional_tests::{checker::check, evaluator::eval, utils::parse_input};
use sprint_move::generator::Generator;
use sprint_parser::ast::contract::Visitor;
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

    fs::create_dir_all("tests/generated").unwrap();
    let generated_file = format!("tests/generated/{}", &file_name);
    let mut file = File::create(&generated_file).unwrap();
    file.write_all(input.as_bytes()).unwrap();

    let (config, directives, transactions) = parse_input(&input).unwrap();
    let log = eval(&config, &transactions).unwrap();

    if let Err(err) = check(&log, &directives) {
        fs::create_dir_all("tests/generated/FAILED").unwrap();
        // Moves the file to the FAILED subdirectory.
        fs::rename(
            &generated_file,
            format!("tests/generated/FAILED/{}", &file_name),
        )
        .unwrap();
        println!("{}", log);
        panic!(err);
    }
}

#[test]
fn zero() {
    let suite = Path::new("tests/contracts/zero.test.mvir");

    let mut generator = Generator::new("Zero");
    generator.visit_zero();

    test(generator.contract(), suite);
}

#[test]
fn one() {
    let suite = Path::new("tests/contracts/one.test.mvir");

    let mut generator = Generator::new("One");
    generator.visit_one();

    test(generator.contract(), suite);
}
