mod day0;
mod day1;
mod day2;
mod day3;

type Solution = fn(&str) -> (String, String);

pub static DAYS : [Solution; 4] = [
    day0::day0,
    day1::day1,
    day2::day2,
    day3::day3,
];
