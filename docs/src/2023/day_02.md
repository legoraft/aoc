# Day two

For day two of advent of code, we arrive at Snow Island. We walk a bit with an elf and play a game. The elf will show us some red, green and blue cubes and we have to tell which games were possible.

## Part one

For the first part, we are given an example input and need to fins out which games were possible with 12 red, 13 green, and 14 blue cubes. Let's start and write a test for that.

```rust,noplayground
#[test]
fn test_part_one() {
    let input_file: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    let answer: i64 = 8;

    assert_eq!(answer, part_one(input_file));
}
```

Make sure to wrap this in `mod tests`, which uses `#[cfg(test)]`. Now, we can start by solving the puzzle. The advent of code explanation says that you need to take a look at every draw from the bag and need to take a look what was the highest number and check if the game was possible. Because we just need to check if the largest number per color is smaller than the given colors, we can skip the whole draw part.

I've tried to compile this into a simple parser, where we're left with a vector that contains all the drawn cubes in a game.

```rust,noplayground
fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines()
        .map(|l| { 
            let (_, cubes) = l
                .split_once(": ")
                .expect("Couldn't split line!"); cubes })
        .collect();

    lines
}
```

This function splits the file in lines and removes everything in front of the `: `, which means we only have the numbers and cubes.