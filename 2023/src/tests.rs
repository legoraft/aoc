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
    let games = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    assert_eq!(8, day_02::part_one(games));
    assert_eq!(2286, day_02::part_two(games));
}

#[test]
fn day_03() {
    let schematic = "\
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    assert_eq!(4361, day_03::part_one(schematic));
}

#[test]
fn day_04() {
    let cards = "\
Card   1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card   2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card   3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card   4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card   5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card   6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    assert_eq!(13, day_04::part_one(cards));
}

#[test]
fn day_05() {
    let almanac = "\
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    assert_eq!(35, day_05::part_one(almanac));
}

#[test]
fn day_06() {
    let race = "\
Time:      7  15   30
Distance:  9  40  200
";

    assert_eq!(288, day_06::part_one(race))
}