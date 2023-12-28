mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day8;
mod day9;

use crate::day1::day1;
use crate::day2::day2;
use crate::day3::day3;
use crate::day4::{day4, day4_part2};
use crate::day5::{day5, day5_part2};
use crate::day6::{day6, day6_part2};
use crate::day8::{day8, day8_part2};
use crate::day9::{day9, day9_part2};

fn main() {
    day1();
    day2();
    day3();
    day4();
    day4_part2();
    day5();
    day5_part2();
    day6();
    day6_part2();
    day8();
    day8_part2();
    day9();
    day9_part2();
}