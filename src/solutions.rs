mod day0;
mod day1;
mod day2;
mod day3;

pub static DAYS : [fn(&str) -> (String, String); 4] = [
    day0::day0,
    day1::day1,
    day2::day2,
    day3::day3,
];
