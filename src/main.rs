mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), DynError> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }

    Ok(())
}

/// match by shifting one character from the beginning of each line,
/// and considered matched if it matches any of the shift.
///
/// for example, if there is a string "abcd", matching is done in the following order,
/// and if any of these matches, the line is considered to match the given regular expression.
///
/// - abcd
/// - bcd
/// - cd
/// - d
fn match_file(expr: &str, file: &str) -> Result<(), DynError> {
    let f = File::open(file)?;
    let reader = BufReader::new(f);

    engine::print(expr)?;
    println!();

    for line in reader.lines() {
        let line = line?;
        for (i, _) in line.char_indices() {
            if engine::do_matching(expr, &line[i..], true)? {
                println!("{line}");
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        engine::do_matching,
        helper::{safe_add, SafeAdd},
    };

    #[test]
    fn test_safe_add() {
        let n = 10usize;
        assert_eq!(Some(30), n.safe_add(&20));

        let n = !0usize;
        assert_eq!(None, n.safe_add(&1));

        let mut n = 10usize;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n = !0usize;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        for use_dfs in [true, false] {
            // parse error
            assert!(do_matching("+b", "bbb", use_dfs).is_err());
            assert!(do_matching("*b", "bbb", use_dfs).is_err());
            assert!(do_matching("|b", "bbb", use_dfs).is_err());
            assert!(do_matching("?b", "bbb", use_dfs).is_err());

            // parse ok, match success
            assert!(do_matching("abc|def", "def", use_dfs).unwrap());
            assert!(do_matching("(abc)*", "abcabc", use_dfs).unwrap());
            assert!(do_matching("(ab|cd)+", "abcdcd", use_dfs).unwrap());
            assert!(do_matching("abc?", "ab", use_dfs).unwrap());

            // parse ok, match fail
            assert!(!do_matching("abc|def", "efa", use_dfs).unwrap());
            assert!(!do_matching("(ab|cd)", "", use_dfs).unwrap());
            assert!(!do_matching("abc?", "acb", use_dfs).unwrap());
        }
    }
}
