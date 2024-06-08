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

This function splits the file in lines and removes everything in front of the `: `, which means we only have the numbers and cubes. Now we need to check which games were possible. I'm starting by creating a loop that registers the indices of a game and maps every game to a `vec` of just the count and the color of the cubes.

```rust,noplayground
for (id, game ) in games.iter().enumerate() {
    let id = id + 1;

    let draws: Vec<&str> = game
        .split([',', ';'])
        .map(|s| s.trim())
        .collect();
}
```

This registers every id by using `.iter().enumerate()`. We need to add 1 to that, because the initial id's will be the index of the game, which start at 0. The draws are splitted in every count and cube and the whitespace is trimmed. We split at two different characters, because the draws can be split by those characters (due to draws and sets being a different thing in the explanation).

For every draw, we need to check the color, the amount and if the draw is smaller than the maximum amount possible. If it isn't smaller, we need to skip the id and continue to the next game. The first part is done like so:

```rust,noplayground
for draw in draws {
    let (count, color) = draw
        .split_once(" ")
        .expect("Can't split draw!");

    let count = count.parse::<i64>().expect("Can't parse count!");

    let possible = match color {
        "red" =>  count <= 12,
        "green" =>  count <= 13,
        "blue" =>  count <= 14,
        _ => panic!("That shouldn't happen..."),
    };
}
```

We split the draw into a count and color. which are separated by a space. The count always precedes the color, as you can see in the example. After this, we parse the count so we can compare it easily and return a boolean if the value we've found is possible or not.

We now know if a draw was possible, but we still need to skip the round if the value wasn't possible. This is were the `continue` function comes in. We can use continue to skip the current iteration of a for loop and go to the second.

``` rust,noplayground
'outer: for (id, game ) in games.iter().enumerate() {
    //snip

    for draw in draws {
        //snip

        if !possible { continue 'outer; }
    }

    answer += id as i64;
}
```

We use a label to specify which loop has to continue, which in this case is the outer loop. If we hadn't used this, the inner for loop would just continue at the next draw, making it so every id is added to our answer. If all draws in a game are possible, we add the id to the answer and the next game is checked. At the end of the for loop we also return the answer, giving us access to it in the `main()` function.

Speaking of the `main()` function, it is built up in the same way as day one:

```rust,noplayground
fn main() {
    let input_file = include_str!("../../inputs/day_02.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}
```

We can now use this to print out the answer of part one, but let's add the part two function too!

## Part two

