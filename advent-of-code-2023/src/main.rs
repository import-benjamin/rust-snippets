use crate::solutions::day_01;
use crate::solutions::day_02;

mod solutions;
mod utils;

fn main() {
    let day = 2_i32;

    print!("day {:0>2}: ", day);

    match 2 {
        1 => {
            day_01::step_two(crate::utils::get_input(1));
            day_01::step_one(crate::utils::get_input(1));
        }
        2 => {
            println!("{}", day_02::step_one(crate::utils::get_input(2)));
            println!("{}", day_02::step_two(crate::utils::get_input(2)));
        }
        _ => todo!(),
    }
}
