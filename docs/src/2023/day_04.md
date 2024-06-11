# Day one

On day four, we find an elf sitting in a huge pile of scratchcards. We need to help the elf figure out what he has won.

## Part one

For the first part, we need to calculate the score for the scratchcards. Each card contains two rows of values, separated by a pipe (`|`). The first row are the winning numbers and the second are the numbers on your card. You calculate the score by taking 1 point for the first winning number and doubling this for every consecutive one. This is the same as raising the `score - 1` to the power of 2.

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

This parser is basically the same as in [day 2](day_02.md#part-one). Now, we need to split the winning numbers and the owned numbers and throw them in a `HashSet`. We can use the great `intersection()` function for this day again, just like in [day 3](day_03.md#part-one). So let's map these numbers in a `HashSet`.

```rust,noplayground
for line in lines {
    let (winning, card) = line
        .split_once(" | ")
        .expect("Couldn't split numbers!");

    let winning_numbers: HashSet<i64> = winning
        .split_whitespace()
        .map(|num| num.parse::<i64>().expect("Can't parse number!"))
        .collect();
#   
#   let card_numbers: HashSet<i64> = card
#       .split_whitespace()
#       .map(|num| num.parse::<i64>().expect("Can't parse number!"))
#       .collect();
}
```

First, we split the line at the pipe, so we have two different rows of numbers. Next, we split the cards at the whitespace (because some single-digit numbers have two spaces in front of them) and parse them. After we've parsed the numbers, we map them to a `HashSet`. I've only shown the first variable assignment here, because they're the same. You can take a look at the second one by clicking on the eye icon.

Now, we need to count how many wins we get per card and raise that (minus one) to a power of 2.

```rust,noplayground
for line in lines {
    // snip

    let power = winning_numbers.intersection(&card_numbers).count();

    if power > 0 {
        answer += 2_i64.pow((power - 1) as u32);
    }
}
```

Here, we count the amount of intersections we have per card. Next, we check if the power is greater than 0 (otherwise we're subtracting with overflow) and add the power minus one to the answer. After we've iterated over all the cards, we have our final answer! Now on to part two.

## Part two

Part two makes this a whole lot more difficult. If we win on a card, we get that amount of consecutive copies on the next crads. There is a lot of text explaining this, but the gist of it is as follows: If we win 2 times on card 1, we get 1 copy of card 2 and 1 copy of card 3. Now, we need to calculate the score for the amount of cards we have for the next and the score of every copy also counts towards the amount of consecutive cards we get.

This is really difficult to understand and the solution is everything but difficult.

```rust,noplayground
let mut answer: Vec<i64> = vec![1; lines.len()];

for (index, line) in lines.iter().enumerate() {
    // snip

    for i in index + 1..index + count + 1 {
        answer[i] += answer[index];
    }
}

answer.iter().sum()
```

The for loop is almost exactly the same, we've just added a small inner loop. What this loop does is keep track of how many cards we have in total. We start by creating a `vec!` that contains the value one over the length of all the cards. This can be done with the `vec![1; num]` syntax, where num is the amount of cards, or `lines.len()`.

After calculating the win count, we iterate over the vector we've created from the card after the current card, till the amount of wins we've gotten. We add the value of the current card to the cards at that location, essentially creating the correct amount of duplicates for that card.

If that was a bit unclear, maybe this will make it a bit more clear. If we have a vector of 3 cards, which looks like `[1, 1, 1]` when initialized, because we start with one card each. Then card 1 has 2 wins, giving us an extra card 2 and 3. The iterator will go from the location of card 1 and add it to card 2 and 3, creating `[1, 2, 2]` as a vec. Next we have card 2, which has 1 win and gives us an extra card 3. The iterator will now take the value of card 2 (which is 2, because we have one copy) and add this to card 3. This means that our final vector will be `[1, 2, 3]`. By summing this, we get our total amount of cards!

This day was a lot of parsing and iterating, so if you want to check out the files, take a look in the [repo](https://github.com/legoraft/aoc/blob/main/2023/day_04). If you want to test the full solution with the test input, check out the [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=4b16490c5f91cc5f9d7cf96272170830).