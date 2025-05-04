use regex::Regex;

fn translate_value(x: &str) -> Option<u32> {
    let res = match x {
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => 0,
    };
    Some(res)
}
/// Process the given line to extract first and last numerical value.
fn process_line(line: String) -> u32 {
    let head_regex = Regex::new(r"(?<number>[1-9])").expect("Invalid Regex.");
    let last_regex = Regex::new(r"(?:.*)(?<number>[1-9])").expect("Invalid Regex.");

    let head_digit = head_regex
        .find(line.as_str())
        .and_then(|m| Some(m.as_str()))
        .and_then(translate_value);
    let last_digit = last_regex
        .captures(line.as_str())
        .and_then(|m| m.name("number"))
        .and_then(|m| Some(m.as_str()))
        .and_then(translate_value);

    head_digit.unwrap() * 10 + last_digit.unwrap()
}

fn process_line_bis(line: String) -> u32 {
    let head_regex = Regex::new(r"(?<number>[1-9]|one|two|three|four|five|six|seven|eight|nine)")
        .expect("Invalid Regex.");
    let last_regex =
        Regex::new(r"(?:.*)(?<number>[1-9]|one|two|three|four|five|six|seven|eight|nine)")
            .expect("Invalid Regex.");

    let head_digit = head_regex
        .find(line.as_str())
        .and_then(|m| Some(m.as_str()))
        .and_then(translate_value);
    let last_digit = last_regex
        .captures(line.as_str())
        .and_then(|m| m.name("number"))
        .and_then(|m| Some(m.as_str()))
        .and_then(translate_value);

    head_digit.unwrap() * 10 + last_digit.unwrap()
}

pub fn step_one(input: String) -> u32 {
    input.lines().map(str::to_string).map(process_line).sum()
}

pub fn step_two(input: String) -> u32 {
    input
        .lines()
        .map(str::to_string)
        .map(process_line_bis)
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("7pqrstfiveteen", 75)]
    #[case("eighthree", 83)]
    #[case("sevenine", 79)]
    #[case("treb7uchet", 77)]
    fn compute_step_two_examples(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(
            crate::solutions::day_01::step_two(input.to_string()),
            expected
        )
    }

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    fn compute_step_one_examples(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(
            crate::solutions::day_01::step_one(input.to_string()),
            expected
        )
    }
}
