mod engine;
mod helper;

use helper::DynError;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader},
};

/// Open a file and do matching per line
///
/// マッチングはそれぞれの行頭から1文字ずつずらして行い、
/// いずれかマッチした場合に、その行がマッチしたものとみなす
///
/// 例えば、abcdという文字列があった場合、以下の順にマッチが行われ、
/// このいずれかにマッチした場合、与えられた正規表現にマッチする行と判定する
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
fn main() -> Result<(), DynError> {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 2 {
        eprintln!("usage: {} regex file", args[0]);
        return Err("invalid arguments".into());
    } else {
        match_file(&args[1], &args[2])?;
    }
    Ok(())
}

// unit tests
#[cfg(test)]
mod tests {
    use crate::{
        engine::{do_matching, do_matching_with_cache},
        helper::{safe_add, SafeAdd},
    };
    #[test]
    fn test_safe_add() {
        let n: usize = 10;
        assert_eq!(Some(30), n.safe_add(&20));

        let n: usize = !0; // 2^64 - 1 (64 bits CPU)
        assert_eq!(None, n.safe_add(&1));

        let mut n: usize = 10;
        assert!(safe_add(&mut n, &20, || ()).is_ok());

        let mut n: usize = !0;
        assert!(safe_add(&mut n, &1, || ()).is_err());
    }

    #[test]
    fn test_matching() {
        // Parse Errors
        assert!(do_matching("+b", "bbb", true).is_err());
        assert!(do_matching("*b", "bbb", true).is_err());
        assert!(do_matching("|b", "bbb", true).is_err());
        assert!(do_matching("?b", "bbb", true).is_err());

        // Parse OK and Match OK
        assert!(do_matching("abc|def", "def", true).unwrap());
        assert!(do_matching("(abc)*", "abcabc", true).unwrap());
        assert!(do_matching("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(do_matching("abc?", "ab", true).unwrap());

        // Parse OK and Match NG
        assert!(!do_matching("abc|def", "efa", true).unwrap());
        assert!(!do_matching("(ab|cd)+", "", true).unwrap());
        assert!(!do_matching("abc?", "acb", true).unwrap());
    }
    #[test]
    fn test_matching_with_cache() {
        // Parse Errors
        assert!(do_matching_with_cache("+b", "bbb", true).is_err());
        assert!(do_matching_with_cache("*b", "bbb", true).is_err());
        assert!(do_matching_with_cache("|b", "bbb", true).is_err());
        assert!(do_matching_with_cache("?b", "bbb", true).is_err());

        // Parse OK and Match OK
        assert!(do_matching_with_cache("abc|def", "def", true).unwrap());
        assert!(do_matching_with_cache("(abc)*", "abcabc", true).unwrap());
        assert!(do_matching_with_cache("(ab|cd)+", "abcdcd", true).unwrap());
        assert!(do_matching_with_cache("abc?", "ab", true).unwrap());

        // Parse OK and Match NG
        assert!(!do_matching_with_cache("abc|def", "efa", true).unwrap());
        assert!(!do_matching_with_cache("(ab|cd)+", "", true).unwrap());
        assert!(!do_matching_with_cache("abc?", "acb", true).unwrap());
    }
}
