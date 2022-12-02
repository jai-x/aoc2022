mod day0;
mod day1;
mod day2;

pub static DAYS : [fn(&str) -> (String, String); 3] = [
    day0::day0,
    day1::day1,
    day2::day2,
];
