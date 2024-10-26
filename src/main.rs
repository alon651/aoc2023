use days::day5;
use std::fmt::Display;
fn main() {
    calculate_time(day5::part1);
    calculate_time(day5::part2);
}

fn calculate_time<T, F>(f: F)
where
    F: Fn() -> T,
    T: Display,
{
    let start_part_one = std::time::Instant::now();
    let result_part_one = f();
    let duration_part_one = start_part_one.elapsed();
    println!("result: {}, Time: {:?}", result_part_one, duration_part_one);
}
