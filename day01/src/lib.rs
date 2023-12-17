/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
///
/// ```
/// assert_eq!(day01::extract_number("abc1def2"), 12);
/// assert_eq!(day01::extract_number("1"), 11);
/// assert_eq!(day01::extract_number(""), 0);
/// assert_eq!(day01::extract_number("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number(s: &str) -> u32 {
    let numbers = s.chars().filter_map(|s| s.to_digit(10)).collect::<Vec<_>>();
    let result = match (numbers.first(), numbers.last()) {
        (Some(first), Some(last)) => (first * 10) + last,
        _ => 0,
    };
    result
}

/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// This version was written purposely in imperative style
///
/// ```
/// assert_eq!(day01::extract_number2("abc1def2"), 12);
/// assert_eq!(day01::extract_number2("1"), 11);
/// assert_eq!(day01::extract_number2(""), 0);
/// assert_eq!(day01::extract_number2("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number2(s: &str) -> u32 {
    let mut first = None;
    let mut last = None;
    for c in s.chars() {
        if let Some(n) = c.to_digit(10) {
            match (first, last) {
                (None, _) => first = Some(n),
                _ => last = Some(n),
            }
        }
    }
    match (first, last) {
        (Some(first), Some(last)) => (first * 10) + last,
        (Some(first), None) => (first * 10) + first,
        _ => 0,
    }
}

/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// This version is an attempt to optimize the extract_number
///
/// ```
/// assert_eq!(day01::extract_number3("abc1def2"), 12);
/// assert_eq!(day01::extract_number3("1"), 11);
/// assert_eq!(day01::extract_number3(""), 0);
/// assert_eq!(day01::extract_number3("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number3(s: &str) -> u32 {
    let mut numbers = s.chars().filter_map(|s| s.to_digit(10));
    let first = numbers.nth(0);
    let last = numbers.last();
    match (first, last) {
        (Some(first), Some(last)) => (first * 10) + last,
        (Some(first), None) => (first * 10) + first,
        _ => 0,
    }
}

/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// This version is an attempt to go as fast as possible
///
/// ```
/// assert_eq!(day01::extract_number4("abc1def2"), 12);
/// assert_eq!(day01::extract_number4("1"), 11);
/// assert_eq!(day01::extract_number4(""), 0);
/// assert_eq!(day01::extract_number4("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number4(s: &str) -> u32 {
    let bytes = s.bytes();
    let setyb = bytes.clone().rev();
    let mut first = 0;
    let mut last = 0;
    for b in bytes {
        if b < 58 && b > 47 {
            first = b - 48;
            break;
        }
    }
    for b in setyb {
        if b < 58 && b > 47 {
            last = b - 48;
            break;
        }
    }
    ((first * 10) + last) as u32
}

/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// This version is an attempt to go as fast as possible, but even faster than that
/// Some observations:
///     using a closure is just as performant so you can DRY up the code a bit
///     rust lacks partial application, attempting to DRY up the iter chain is painful
///     since iterators have explicit types. The only way I could make it work is using `&mut dyn Iterator<Item = u8>`:
///     ex: let get_first_number = |iter: &mut dyn Iterator<Item = u8>| iter.filter_map(|b| if b < 58 && b > 47 { Some(b - 48) } else { None }).nth(0).unwrap_or(0);
///         let first = get_first_number(&mut s.bytes());
///         let last = get_first_number(&mut s.bytes().rev());
///         ...
///     Unfortunately on top of it being ugly, it's also slower.
///
/// ```
/// assert_eq!(day01::extract_number5("abc1def2"), 12);
/// assert_eq!(day01::extract_number5("1"), 11);
/// assert_eq!(day01::extract_number5(""), 0);
/// assert_eq!(day01::extract_number5("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number5(s: &str) -> u32 {
    let is_ascii_number = |b| if b < 58 && b > 47 { Some(b - 48) } else { None };
    let first = s.bytes().filter_map(is_ascii_number).nth(0).unwrap_or(0);
    let last = s
        .bytes()
        .rev()
        .filter_map(is_ascii_number)
        .nth(0)
        .unwrap_or(0);
    ((first * 10) + last) as u32
}

/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// This version is an attempt to go as fast as possible, but even faster than that, but without resorting to byte hacks
///
/// ```
/// assert_eq!(day01::extract_number6("abc1def2"), 12);
/// assert_eq!(day01::extract_number6("1"), 11);
/// assert_eq!(day01::extract_number6(""), 0);
/// assert_eq!(day01::extract_number6("a1b2c3d4e5"), 15);
/// ```
pub fn extract_number6(s: &str) -> u32 {
    let is_number = |c: char| c.to_digit(10);
    let first = s.chars().filter_map(is_number).nth(0).unwrap_or(0);
    let last = s.chars().rev().filter_map(is_number).nth(0).unwrap_or(0);
    ((first * 10) + last) as u32
}

use regex::Regex;
/// Given a string s, returns a 2 digit number comprised of the first and last numbers in s
///
/// If s does not contain any numbers 0 will be returned
/// If s contains a single number, it will be used as both digits
/// A number is either a digit 0-9 or a string "one", "two", "three", etc.
/// There is probably a better way to solve this but this is good enough
///
/// ```
/// assert_eq!(day01::extract_number_ex("1twone"), 11);
/// ```
pub fn extract_number_ex(s: &str) -> u32 {
    let re = Regex::new(r"(?<digit>[0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let re2 = Regex::new(r"(?<digit>[0-9]|eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let s2 = s.chars().rev().collect::<String>();
    let mut captures_forward = re.captures_iter(s);
    let mut captures_backward = re2.captures_iter(&s2);
    let first = captures_forward.nth(0).and_then(|c| c.name("digit"));
    let last = captures_backward.nth(0).and_then(|c| c.name("digit"));
    match (first, last) {
        (Some(first_match), Some(last_match)) => {
            parse_number(first_match.as_str()) * 10 + parse_number(last_match.as_str())
        }
        (Some(first_match), None) => {
            parse_number(first_match.as_str()) * 10 + parse_number(first_match.as_str())
        }
        _ => 0,
    }
}

fn parse_number(s: &str) -> u32 {
    match s {
        "0" => 0,
        "1" | "one" | "eno" => 1,
        "2" | "two" | "owt" => 2,
        "3" | "three" | "eerht" => 3,
        "4" | "four" | "ruof" => 4,
        "5" | "five" | "evif" => 5,
        "6" | "six" | "xis" => 6,
        "7" | "seven" | "neves" => 7,
        "8" | "eight" | "thgie" => 8,
        "9" | "nine" | "enin" => 9,
        _ => 0,
    }
}
