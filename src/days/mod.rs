mod day1;
mod day5;
mod day8;
mod day9;

use counted_array::counted_array;

counted_array!(
    pub const ALL_DAYS: [fn() -> (String, String); _] = [
        day1::run,
        day5::run,
        day8::run,
        day9::run
    ]
);
