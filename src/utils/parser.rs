pub fn parse_const<'a>(s: &'a str, expected: &str) -> Option<(&'a str, &'a str)> {
    s.strip_prefix(expected).map(|x| (&s[..expected.len()], x))
}

pub fn parse_unsigned_int(s: &str) -> Option<(u32, &'_ str)> {
    let x = s
        .chars()
        .take_while(|c: &char| c.is_ascii_digit())
        .collect::<String>();
    Some((x.parse().ok()?, &s[x.len()..]))
}

pub fn parse_int(s: &str) -> Option<(i32, &'_ str)> {
    let (negative, s) = match parse_const(s, "-") {
        Some((_, rest)) => (-1, rest),
        None => (1, s),
    };
    let (n, s) = parse_unsigned_int(s)?;
    Some(((n as i32) * negative, s))
}

pub fn parse_collection<'a, T>(
    s: &'a str,
    seprator: &str,
    parser: impl Fn(&'a str) -> Option<(T, &'a str)>,
) -> Option<(Vec<T>, &'a str)> {
    let (v, s) = parser(s)?;
    let s2 = s.trim_start();
    let s = match parse_const(s2, seprator) {
        Some((_, s)) => s.trim_start(),
        None => return Some((vec![v], s)),
    };
    match parse_collection(s, seprator, parser) {
        Some((mut values, s)) => {
            values.insert(0, v);
            Some((values, s))
        }
        None => Some((vec![v], s)),
    }
}

pub fn parse_delimited<'a, T>(
    s: &'a str,
    open: &str,
    close: &str,
    trim: bool,
    parser: impl Fn(&'a str) -> Option<(T, &'a str)>,
) -> Option<(T, &'a str)> {
    let s = match trim {
        true => parse_const(s, open).map(|(_, s)| s.trim_start())?,
        false => parse_const(s, open).map(|(_, s)| s)?,
    };
    let (v, s) = parser(s)?;
    let s = match trim {
        true => parse_const(s.trim_start(), close).map(|(_, s)| s)?,
        false => parse_const(s, close).map(|(_, s)| s)?,
    };
    Some((v, s))
}

pub fn parse_eof(s: &str) -> Option<((), &'_ str)> {
    s.is_empty().then_some(((), s))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_const() {
        assert_eq!(
            parse_const("helloworld", "hello").unwrap(),
            ("hello", "world")
        );
        assert!(parse_const("worldhello", "hello").is_none())
    }
    #[test]
    fn test_parse_unsigined_int() {
        assert_eq!(parse_unsigned_int("1234hello").unwrap(), (1234, "hello"));
        assert_eq!(parse_unsigned_int("01234hello").unwrap(), (1234, "hello"));
    }
    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int("1234hello").unwrap(), (1234, "hello"));
        assert_eq!(parse_int("-1234hello").unwrap(), (-1234, "hello"));
        assert_eq!(parse_int("-01234hello").unwrap(), (-1234, "hello"));
        assert_eq!(parse_int("01234hello").unwrap(), (1234, "hello"));
    }
    #[test]
    fn test_parse_collection() {
        assert_eq!(
            parse_collection("1, 2, 3", ",", parse_int).unwrap(),
            (vec![1, 2, 3], "")
        );
        assert_eq!(
            parse_collection("1, 2, 3abcde", ",", parse_int).unwrap(),
            (vec![1, 2, 3], "abcde")
        );
        assert_eq!(
            parse_collection("1 , 2 , 3 ", ",", parse_int).unwrap(),
            (vec![1, 2, 3], " ")
        )
    }
    #[test]
    fn test_parse_delimited() {
        assert_eq!(
            parse_delimited("(hello)", "(", ")", true, |x| parse_const(x, "hello")).unwrap(),
            ("hello", "")
        );
        assert_eq!(
            parse_delimited("(  hello     )", "(", ")", true, |x| parse_const(
                x, "hello"
            ))
            .unwrap(),
            ("hello", "")
        );
        assert_eq!(
            parse_delimited("(hello)", "(", ")", false, |x| parse_const(x, "hello")).unwrap(),
            ("hello", "")
        );
        assert!(
            parse_delimited("(  hello     )", "(", ")", false, |x| parse_const(
                x, "hello"
            ))
            .is_none(),
        );
        assert_eq!(
            parse_delimited("[1,2,3]", "[", "]", true, |x| parse_collection(
                x, ",", parse_int
            ))
            .unwrap(),
            (vec![1, 2, 3], "")
        )
    }
}
