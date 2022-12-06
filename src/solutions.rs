mod day0;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

type Solution = fn(&str) -> (String, String);

pub static DAYS: [Solution; 7] = [
    day0::day0,
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
    day5::day5,
    day6::day6,
];
