use regex::Regex;
use std::cmp::{max, Ordering};

#[derive(Debug, Copy, Clone)]
struct Game {
    id: u32,
    blue: u32,
    green: u32,
    red: u32,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Compare the fields and return the appropriate Ordering
        if self.blue > other.blue && self.green > other.green && self.red > other.red {
            Some(Ordering::Greater)
        } else if self.blue < other.blue || self.green < other.green || self.red < other.red {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
    }
}

fn process_game_properties(game_line: String) -> Game {
    let game_id_regex = Regex::new(r"Game (?P<id>\d+):").expect("Invalid game id regex");
    let game_props_regex =
        Regex::new(r"(?P<count>\d+) (?P<color>blue|green|red)").expect("Invalid Regex");

    let found_game_id: u32 = game_id_regex
        .captures(game_line.as_str())
        .expect("Game id not found")
        .name("id")
        .expect("Group does not exist")
        .as_str()
        .parse::<u32>()
        .expect("Can't parse game Id");

    let mut blue_count = 0_u32;
    let mut green_count = 0_u32;
    let mut red_count = 0_u32;

    for c in game_props_regex.captures_iter(game_line.as_str()) {
        let color: &str = c.name("color").expect("Error with color").as_str();
        let value: u32 = c
            .name("count")
            .expect("Error with count")
            .as_str()
            .parse::<u32>()
            .expect("This is not an u32 value");

        match color {
            "blue" => blue_count = max(blue_count, value),
            "green" => green_count = max(green_count, value),
            "red" => red_count = max(red_count, value),
            _ => {}
        }
    }
    Game {
        id: found_game_id,
        blue: blue_count,
        green: green_count,
        red: red_count,
    }
}
pub fn step_one(input: String) -> u32 {
    // Define game reference for comparison
    let game_ref: Game = Game {
        id: 0,
        blue: 14,
        green: 13,
        red: 12,
    };

    str::lines(&input)
        .map(str::to_string)
        // Turn string to Game struct
        .map(process_game_properties)
        // Keep only Games that are less than or equal to the game ref
        .filter(|g| game_ref.ge(g))
        // Extract the compatible games id
        .map(|g| g.id)
        .sum()
}

pub fn step_two(input: String) -> u32 {
    str::lines(&input)
        .map(str::to_string)
        // Turn string to Game struct
        .map(process_game_properties)
        // Compute power
        .map(|g| g.red * g.blue * g.green)
        // Extract the compatible games id
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::solutions::day_02::Game;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[rstest]
    #[case(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        8
    )]
    fn compute_step_one_examples(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(
            crate::solutions::day_02::step_one(input.to_string()),
            expected
        )
    }

    #[rstest]
    fn compute_step_two_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        assert_eq!(crate::solutions::day_02::step_two(input.to_string()), 2286)
    }

    #[rstest]
    #[case(Game { id: 0, blue: 2, green: 3, red: 4}, Some(Ordering::Less))]
    #[case(Game { id: 1, blue: 2, green: 10, red: 10}, Some(Ordering::Less))]
    #[case(Game { id: 2, blue: 4, green: 5, red: 6}, Some(Ordering::Greater))]
    #[case(Game { id: 3, blue: 3, green: 4, red: 5}, Some(Ordering::Equal))]
    fn test_game_comparison(#[case] input: Game, #[case] expected: Option<Ordering>) {
        let reference = Game {
            id: 0,
            blue: 3,
            green: 4,
            red: 5,
        };

        assert_eq!(input.partial_cmp(&reference), expected);
    }
}
