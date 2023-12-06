use std::time::Instant;

pub mod days;

#[cfg(test)]
mod tests;

fn output_part(part_one: impl FnOnce() -> i64, part_two: impl FnOnce() -> i64, day: &str) -> String {
    let (part_one_result, part_one_duration) = timed(part_one);
    let (part_two_result, part_two_duration) = timed(part_two);

    let part_one = format_part(part_one_result, part_one_duration);
    let part_two = format_part(part_two_result, part_two_duration);

    let output = format!("\
---- 2023, Day {day} ----
{part_one}
{part_two}");

    output
}

fn format_part(answer: i64, duration: String) -> String {
    let output_string = format!("\
{duration}s Part 1: {:?}", answer);

    output_string
}

fn timed(body: impl FnOnce() -> i64) -> (i64, String) {
    let start = Instant::now();
    let result = body();
    let elapsed_ms = start.elapsed().as_millis();

    let seconds = elapsed_ms / 1000;
    let milliseconds = elapsed_ms % 1000;

    let duration = format!("{seconds:3}.{milliseconds:03}");

    (result, duration)
}