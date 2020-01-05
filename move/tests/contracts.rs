use functional_tests::{
    checker::*,
    config::global::Config as GlobalConfig,
    evaluator::eval,
    preprocessor::{build_transactions, split_input},
};
use sprint_move::generate;
use sprint_parser::parser::contract;
use std::{
    env,
    fmt::Display,
    fs::{self, File},
    io::{Read, Write},
    iter,
    path::Path,
};
use termcolor::{BufferWriter, Color, ColorChoice, ColorSpec, WriteColor};

fn at_most_n_chars(s: impl IntoIterator<Item = char>, n: usize) -> String {
    let mut it = s.into_iter();
    let mut s = String::new();
    for _ in 0..n {
        match it.next() {
            Some(c) => s.push(c),
            None => return s,
        }
    }
    if it.next().is_some() {
        s.push_str("...")
    }
    s
}

fn at_most_n_before_and_m_after(
    s: &str,
    n: usize,
    start: usize,
    end: usize,
    m: usize,
) -> (String, String, String) {
    let before = at_most_n_chars(s[..start].chars().rev(), n)
        .chars()
        .rev()
        .collect();
    let matched = s[start..end].to_string();
    let after = at_most_n_chars(s[end..].chars(), m).chars().collect();
    (before, matched, after)
}

fn env_var(var_name: &str) -> String {
    env::var(var_name)
        .unwrap_or_else(|_| "".to_string())
        .to_ascii_lowercase()
}

fn pretty_mode() -> bool {
    let pretty = env_var("PRETTY");
    pretty == "1" || pretty == "true"
}

// Runs all tests under the test/testsuite directory.
fn test(module: impl Display, path: &Path) {
    let mut input = String::new();
    let file_name = path.file_name().unwrap().to_str().unwrap();

    input.push_str("//! account: alice, 1000000\n");
    input.push_str("//! account: bob, 1000000\n");
    input.push_str("//! account: chris, 1000000\n\n");

    input.push_str("//! new-transaction\n");
    input.push_str("//! sender: alice\n");

    input.push_str(&format!("{}\n", module));

    let mut suite = File::open(path).unwrap();
    suite.read_to_string(&mut input).unwrap();

    // TODO: Only create file if flag is set.
    fs::create_dir_all("tests/generated").unwrap();
    let generated_file = format!("tests/generated/{}", &file_name);
    let mut file = File::create(&generated_file).unwrap();
    file.write_all(input.as_bytes()).unwrap();

    let lines: Vec<String> = input.lines().map(ToString::to_string).collect();

    let (config, directives, transactions) = split_input(&lines).unwrap();
    let config = GlobalConfig::build(&config).unwrap();
    let transactions = build_transactions(&config, &transactions).unwrap();

    let log = eval(&config, &transactions).unwrap();

    let res = match_output(&log, &directives);

    let errs = match res.status {
        MatchStatus::Success => return,
        MatchStatus::Failure(errs) => errs,
    };

    // Set up colored output stream for error rendering.
    let writer = BufferWriter::stdout(ColorChoice::Auto);
    let mut output = writer.buffer();

    // Helpers for directives and matches.
    macro_rules! print_directive {
        ($idx: expr) => {{
            let d = &directives[$idx];
            write!(output, "{} | {}", d.line + 1, &lines[d.line][..d.start]).unwrap();
            output
                .set_color(ColorSpec::new().set_underline(true))
                .unwrap();
            write!(output, "{}", &lines[d.line][d.start..d.end]).unwrap();
            output.reset().unwrap();
            write!(output, "{}", &lines[d.line][d.end..]).unwrap()
        }};
    }

    macro_rules! print_match {
        ($indent: expr, $is_positive: expr, $m: expr) => {{
            let m: &Match = $m;
            let indent: &str = $indent;
            let prefix = format!("[{}] ", m.entry_id);
            let (before, matched, after) =
                at_most_n_before_and_m_after(&res.text[m.entry_id], 30, m.start, m.end, 50);
            write!(output, "{}", indent).unwrap();
            write!(output, "{}{}", prefix, before).unwrap();
            output
                .set_color(
                    ColorSpec::new()
                        .set_underline(true)
                        .set_fg(Some(if $is_positive {
                            Color::Green
                        } else {
                            Color::Red
                        })),
                )
                .unwrap();
            write!(output, "{}", matched).unwrap();
            output.reset().unwrap();
            writeln!(output, "{}", after).unwrap();

            let offset = prefix.chars().count() + before.chars().count();
            write!(output, "{}", indent).unwrap();
            write!(
                output,
                "{}",
                iter::repeat(' ').take(offset).collect::<String>()
            )
            .unwrap();
            print_directive!(m.pat_id);
            writeln!(output).unwrap()
        }};
    }

    writeln!(output).unwrap();
    writeln!(
        output,
        "{}",
        iter::repeat('=').take(100).collect::<String>()
    )
    .unwrap();
    writeln!(output, "{}", path.display()).unwrap();
    writeln!(output).unwrap();

    // Render the evaluation log.
    output
        .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Yellow)))
        .unwrap();
    write!(output, "info: ").unwrap();
    output.set_color(ColorSpec::new().set_bold(true)).unwrap();
    writeln!(output, "Evaluation Outputs").unwrap();
    output.reset().unwrap();
    if pretty_mode() {
        writeln!(
            output,
            "{}",
            format!("{}", log)
                .lines()
                .map(|line| format!("    {}\n", line))
                .collect::<String>()
        )
        .unwrap();
    } else {
        for (id, entry) in res.text.iter().enumerate() {
            writeln!(output, "    [{}] {}", id, entry).unwrap();
        }
        writeln!(output).unwrap();
        writeln!(
            output,
            "    Note: enable pretty printing by setting 'env PRETTY=1'."
        )
        .unwrap();
        writeln!(output).unwrap();
    }
    writeln!(output).unwrap();

    // Render previously successful matches if any.
    if !res.matches.is_empty() {
        output
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Yellow)))
            .unwrap();
        write!(output, "info: ").unwrap();
        output.set_color(ColorSpec::new().set_bold(true)).unwrap();
        writeln!(output, "Successful Matches").unwrap();
        output.reset().unwrap();
        for m in &res.matches {
            print_match!("    ", true, m);
            writeln!(output).unwrap();
        }
        writeln!(output).unwrap();
    }

    // Render errors.
    for err in errs {
        output
            .set_color(ColorSpec::new().set_bold(true).set_fg(Some(Color::Red)))
            .unwrap();
        write!(output, "error: ").unwrap();
        output.reset().unwrap();
        match err {
            MatchError::UnmatchedErrors(errs) => {
                output.set_color(ColorSpec::new().set_bold(true)).unwrap();
                writeln!(output, "Unmatched Errors").unwrap();
                output.reset().unwrap();
                for id in errs.iter() {
                    write!(output, "    [{}] ", id).unwrap();
                    writeln!(output, "{}", at_most_n_chars(res.text[*id].chars(), 80)).unwrap();
                }
            }
            MatchError::NegativeMatch(m) => {
                output.set_color(ColorSpec::new().set_bold(true)).unwrap();
                writeln!(output, "Negative Match").unwrap();
                output.reset().unwrap();
                print_match!("    ", false, &m);
            }
            MatchError::UnmatchedDirectives(dirs) => {
                output.set_color(ColorSpec::new().set_bold(true)).unwrap();
                writeln!(output, "Unmatched Directives").unwrap();
                output.reset().unwrap();
                for idx in &dirs {
                    write!(output, "    ").unwrap();
                    print_directive!(*idx);
                    writeln!(output).unwrap();
                }
                writeln!(output).unwrap();
                writeln!(output).unwrap();
            }
        }
    }
    writeln!(output).unwrap();
    writer.print(&output).unwrap();

    panic!("test failed")
}

#[test]
fn generated_zero() {
    let suite = Path::new("tests/suites/zero.test.mvir");
    let contract = contract("main = zero").unwrap();

    test(generate(&contract), suite);
}

#[test]
fn generated_one() {
    let suite = Path::new("tests/suites/one.test.mvir");
    let contract = contract("main = one").unwrap();

    test(generate(&contract), suite);
}

#[test]
fn generated_one_and_one() {
    let suite = Path::new("tests/suites/one_and_one.test.mvir");
    let contract = contract("main = and one one").unwrap();

    test(generate(&contract), suite);
}
