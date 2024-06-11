# Day one

On day four, we find an elf sitting in a huge pile of scratchcards. We need to help the elf figure out what he has won.

## Part one

For the first part, we need to calculate the score for the scratchcards. Each card contains two rows of values, separated by a pipe (`|`). The first row are the winning numbers and the second are the numbers on your card.

Let's start by writing a test with the example input and throwing it in `mod tests`.

```rust,noplayground
#[test]
fn test_part_one() {
    let input_file: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    let answer: i64 = 13;

    assert_eq!(answer, part_one(input_file));
}
```

Next, we want to parse the file and throw out the `Card #: `. Let's write a simple parser that returns the lines and throws out that first part.

```rust,noplayground
fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines()
        .map(|l| { 
            let (_, cards) = l
                .split_once(": ")
                .expect("Couldn't split line!"); cards })
        .collect();

    lines
}
```

This parser is basically the same as in [day 2](day_02.md#part-one). Now, we need to split the winning numbers and the owned numbers and throw them in a `HashSet`. We can use the great `intersection()` function for this day again, just like in [day 3](day_03.md#part-one).

## Part two

Part two throws us a bit for a loop, because it appears that there are also numbers hidden in our file with plain text. This looks like this: `two1nine`, which means the answer should be `29`. This is because we need to parse the text numbers into numbers and then take the first and last part of the numbers.


The files for this day are available [here](https://github.com/legoraft/aoc/blob/main/2023/day_01). If you want to test the full solution with the test input, check out the [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8218e04fb1cbd290becce380c8e1ffda).