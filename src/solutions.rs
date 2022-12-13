mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

type Solution = fn(&str) -> (String, String);

pub static DAYS: [Solution; 12] = [
    day0::day0,
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
    day7::day7,
    day8::day8,
    day9::day9,
    day10::day10,
    day11::day11,
];
