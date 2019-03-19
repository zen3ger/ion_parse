mod pesto;
use crate::pesto::Rule;

use clap::{clap_app, crate_version};
use std::io::BufRead;

use pest::Parser;
fn main() {
    let cp = clap_app!(pegtest =>
    (version : crate_version!())
    (about : "Tests how pegs work")
    (author : "Matthew Stoodley,Zen3Ger")
    (@arg rule: -r +takes_value "The rule to look for -- default Statement")
    )
    .get_matches();

    let rule = match cp.value_of("rule").unwrap_or("Main") {
//        "Range" => Rule::Range,
//        "Statement" => Rule::Statement,
//        "Statements" => Rule::Statements,
//        "Path" => Rule::Path,

        _ => Rule::Main,
    };

    println!("Rule = {:?}", rule);

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::with_capacity(32);
    loop {
        stdin.read_line(&mut buf).ok();
        match pesto::Command::parse(rule, &buf) {
            Ok(res) => println!("{}", res),
            Err(e) => println!("{}", e),
        }
        buf.truncate(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pest::iterators::Pair;
    use pest::error::{Error, ErrorVariant};

    #[test]
    fn range_pass() {
        // Ranges have a fix start and end value and an optional step.
        // The step can be an integer or anything that expands to a value
        // that can be parsed into an integer.
        // Start and end values can be single characters or anything that
        // expands to an integer or character.
        let tests = vec![
            (Rule::Range, "1..2..10"),
            (Rule::Range, "-1..-2..-10"),
            (Rule::Range, "1..2...10"),
            (Rule::Range, "1..2..=10"),
            (Rule::Range, "1..10"),
            (Rule::Range, "-1..-10"),
            (Rule::Range, "1...10"),
            (Rule::Range, "1..=10"),
            (Rule::Range, "a...z"),
            (Rule::Range, "a..2..z"),
            (Rule::Range, "a..=z"),
            (Rule::Range, "'a'..\"A\""),
            (Rule::Range, "A..3...Z"),
            (Rule::Range, "$start..$step..$fin"),
            (Rule::Range, "@start..@step...@fin"),
            (Rule::Range, "$method($x $y)..@method(@arr)...$method(@method())"),
            (Rule::Range, "$($x $y)..@(@arr)...$(atom @method('litStr'))"),
        ];

        let mut errs = Vec::new();
        for (index, (rule, input)) in tests.iter().enumerate() {
            match pesto::Command::parse(*rule, &input) {
                Err(err) => errs.push((index, input, err)),
                Ok(passed) => {
                    let span = passed.clone().next().unwrap().as_span();
                    if span.end() != input.len() {
                        let err = Error::new_from_span(
                            ErrorVariant::CustomError { message: "partial match".into() },
                            span
                        );
                        // partial match, should be considered as an error
                        errs.push((index, input, err));
                    }
                }
            }
            if let Err(err) = pesto::Command::parse(*rule, &input) {
                errs.push((index, input, err));
            }
        }

        if errs.len() > 0 {
            for (index, input, err) in errs {
                println!("[{}] {}", index, input);
                println!("{}", err);
            }
            panic!();
        }
    }

    #[test]
    fn range_fail() {
        let tests = vec![
            (Rule::Range, "1a..1"),
            (Rule::Range, "'aaa'..'bbb'"),
            (Rule::Range, "1..z...4"),
            (Rule::Range, "1...z..4"),
            (Rule::Range, "1.."),
            (Rule::Range, "1..ccc"),
        ];

        let mut errs = Vec::new();
        for (index, (rule, input)) in tests.iter().enumerate() {
            if let Ok(passed) = pesto::Command::parse(*rule, &input) {
                let span = passed.clone().next().unwrap().as_span();
                if span.end() == input.len() {
                    // Partial matches can happen, but they count as failure,
                    // complete parsing catches it whit `~ EOI`
                    errs.push((index, input, passed));
                }
            }
        }

        if errs.len() > 0 {
            for (index, input, passed) in errs {
                println!("[{}] {}", index, input);
                println!("{}\n", passed);
            }
            panic!();
        }
    }

    #[test]
    fn should_pass() {
        let v = vec![
            // We cannot check if the assignment count or type is correct
            // just give the infered input to actuall assignment handler
            (Rule::StatementLet, "let x:int y:[[int]] = atom0 atom1"),
//            (Rule::Main, "let x = 4"),
//            (Rule::Main, "let x y = 4 5"),
//            (Rule::Statement, "echo $x"),
//            (Rule::Main, "echo $x"),
//            (Rule::Statement, "echo \"b\""),
//            (Rule::Main, "for x in 0..4;echo $x; end;"),
//            (Rule::Statement, "for x in 0..4\n echo $x\n end"),
//            (
//                Rule::Main,
//                "for x y hotel in 0..100;let b = \"$(x)oo\";echo b; end",
//            ),
//            (Rule::Main, "for x y hotel in 0..100\n end"),
//            (Rule::Statement, r#"let b = "$(x)oo""#),
//            (Rule::Main, "mayfail -p hello && isok"),
//            (Rule::Main, "mayfail p hello && isok"),
//            (Rule::Statement, "echo $build(3 5 9)"),
//            (Rule::Statement, "ls -l"),
//            (Rule::Path, "home/dir/"),
//            (Rule::Main, "home/dir/"),
//            (Rule::Statement, "./home/dir"),
//            (Rule::Statement, "/dev/etc"),
//            (Rule::Statement, "~/Documents/files"),
//            (Rule::Statement, "cd ~/Documents/My\\ Pictures"),
//            (Rule::Range, "0..4"),
//            (Rule::Range, "0...4"),
//            (Rule::Range, "0..3..9"),
//            (Rule::Range, "10..-2..=0"),
//            (Rule::Range, "$(ls -l)"),
//            (Rule::Range, "0..$s"),
        ];

        let mut errs = Vec::new();
        for (n, (rl, st)) in v.iter().enumerate() {
            if let Err(e) = pesto::Command::parse(*rl, &st) {
                errs.push((n, rl, st, e));
            }
        }

        if errs.len() > 0 {
            for e in errs {
                println!("{:?}\n", e);
            }
            panic!();
        }
    }
//    #[test]
//    fn should_fail() {
//        let v = vec![
//            (Rule::Main, "let x & 4"),
//            (Rule::Main, "let x y = 3"),
//            (Rule::Main, "let x y z = 3 4"),
//            (Rule::Main, "let x = 3 4"),
//            (Rule::Main, "let x y z = 3 4 5 2"),
//            (Rule::Main, "for x in ls -l; echo $x; end;"),
//            (Rule::Statement, "for x in [0..4]\n echo $x\n end;"),
//            (Rule::Range, "[0..Green]"),
//            (Rule::Range, "["),
//            (Rule::Range, "$(ls -l)"),
//            (Rule::Path, "home/dir"),
//            (Rule::Main, "home/dir"),
//        ];
//
//        let mut errs = Vec::new();
//        for (n, (rl, st)) in v.iter().enumerate() {
//            if let Ok(v) = pesto::Command::parse(*rl, &st) {
//                errs.push((n, rl, st, v));
//            }
//        }
//
//        if errs.len() > 0 {
//            for e in errs {
//                println!("{:?}\n", e);
//            }
//            panic!();
//        }
//    }
}
