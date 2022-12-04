mod day0;
mod day1;
mod day2;
mod day3;
mod day4;

type Solution = fn(&str) -> (String, String);

pub static DAYS: [Solution; 5] = [
    day0::day0,
    day1::day1,
    day2::day2,
    day3::day3,
    day4::day4,
];
