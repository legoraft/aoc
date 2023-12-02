use super::*;
use days::*;

#[test]
fn day_01() {
    let calibration_value = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    let calibration_value_text = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    assert_eq!(142, day_01::part_one(calibration_value));
    assert_eq!(281, day_01::part_two(calibration_value_text));
}

#[test]
fn day_02() {
    let games = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    assert_eq!(8, day_02::part_one(games))
}