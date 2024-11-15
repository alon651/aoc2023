use days::day5;
use std::fmt::Display;
fn main() {
    calculate_time(day5::part1);
    calculate_time(day5::part2);
    calculate_time(day5::slow);
}

fn calculate_time<T, F>(f: F)
where
    F: Fn() -> T,
    T: Display,
{
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("result: {}, Time: {:?}", result, duration);
}
