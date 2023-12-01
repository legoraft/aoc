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