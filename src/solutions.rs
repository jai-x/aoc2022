mod day0;
mod day1;

pub static DAYS : [fn(&str) -> (String, String); 2] = [
    day0::day0,
    day1::day1,
];
