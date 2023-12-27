use std::env;

pub fn game_input(day: u8) -> String {
    println!("Get input for day {}", day);
    reqwest::blocking::Client::new()
        .get(format!("https://adventofcode.com/2023/day/{}/input", day))
        .header(
            "Cookie",
            format!(
                "session={}",
                env::var("SESSION_TOKEN").expect("$SESSION_TOKEN is not set")
            ),
        )
        .send()
        .unwrap()
        .text()
        .unwrap()
}
