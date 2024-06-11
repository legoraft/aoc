# Day one

On day one, you are loaded in a trebuchet of which the calibration was done by a young artistic elf. Due to the artistry, the calibration value is difficult to read. We need to get the calibration value for the elves.

## Part one

For the first part, we need to find a calibration value within a line of text, grabbing only the first and last number in that line of text. These two numbers have to be concatenated and form a two digit number together. By adding up all these numbers, you find your calibration value.

I started by just creating a simple test to check if the example input equals the example answer. With this I can check if my code is correct for the example, so I can use it on my true input. Tests are really easy to write in rust, so my test looks like this:

```rust,noplayground
#[test]
fn test_part_one() {
    let input_file: &str = "\
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    let answer: i64 = 142;

    assert_eq!(answer, part_one(input_file));
}
```

This will check if the answer is the same as the output of part one. These tests do have to be wrapped in a `mod tests` which is also has a `#[cfg(tests)]` value associated with it.

Next, I added a simple parser that splits the input into lines, so I can iterate over every line within my `part_one()` function. To get all the lines from some text, you can just use the `text.lines()` function and store this in a vec as follows.

```rust,noplayground
fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines().collect();

    lines
}
```

Now let's start with solving the problem!

```rust,noplayground
fn part_one(input: &str) -> i64 {
    let lines = parse(input);

    let mut answer: i64 = 0;

    for line in lines {
        let nums: Vec<char> = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        
        let number_string: String = [nums[0], nums[nums.len() - 1]].iter().collect();
        let number: i64 = number_string.parse::<i64>().expect("Can't parse string!");

        answer += number;
    }

    answer
}
```

In the for loop, we first create a vector from the characters in our string that are digits. This is done by the `filter()` method. Fofr every character `c`, we check if the number is a digit within the range of 0-10. If this is true, the character is collected by the vector. So if a string is `1abc2`, the associated vector will be `['1', '2']`.

Next up, we create a `String` from the characters. We need the first and last character in the vector, so position 0 and the length of the vector -1, because the length doesn't start at 0. By collecting these characters, we can create a string, which is essentially a vector of characters.

At last, we parse the string, which always should be possible. We `panic!` if it isn't possible, because that means that something went wrong. The final number is added to the answer and this is repeated for every line in the file. By returning the number, we can just use `println!` to print the number and get our answer. This is exactly what we do in the main function.

```rust,noplayground
fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}
```

Here we read the input file to a string and use this as input for our `part_one()` function. This function returns an `i64`, so we bind a variable to that and print the variable. If everything works correctly, you should see the correct answer here.

## Part two

Part two throws us a bit for a loop, because it appears that there are also numbers hidden in our file with plain text. This looks like this: `two1nine`, which means the answer should be `29`. This is because we need to parse the text numbers into numbers and then take the first and last part of the numbers.

Let's start by writing another test and making sure all the functions are correct. Our new test uses a different test input, so it looks a bit different.

```rust,noplayground
#[test]
fn test_part_two() {
    let input_file: &str = "\
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    let answer: i64 = 281;

    assert_eq!(answer, part_two(input_file));
}
```

We also have created a `part_two()` function, which returns an `i64` and added the part two output to our `main()` function. This looks like this.

```rust,noplayground
fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");

    let answer_one = part_one(input_file);
    let answer_two = part_two(input_file);

    println!("Part one: {}\nPart two: {}", answer_one, answer_two);
}

fn part_two(input: &str) -> i64 {
    let lines = parse(input);

    281
}
```

We still use the same input file, but the output now also outputs the second part. I'm currently just returning the answer so my test passes. Now let's start solving the problem. 

Luckily, we can use most of the code from part one. I know that repetition is bad practice, but I want to have every part work independently and be clear to read for every function. So I've copied all the code over from the first part.

```rust,noplayground
fn part_one(input: &str) -> i64 {
    let lines = parse(input);

    let mut answer: i64 = 0;

    for line in lines {
        let nums: Vec<char> = line.chars()
            .filter(|c| c.is_digit(10))
            .collect();
        
        let number_string: String = [nums[0], nums[nums.len() - 1]].iter().collect();
        let number: i64 = number_string.parse::<i64>().expect("Can't parse string!");

        answer += number;
    }

    answer
}
```

This function works exactly like the one in the first part, but we need to make some modifications to the `line` variable before we parse it. This is quite easy, as we can just replace any occurence of a string in a line with another string with `replace()`.

```rust,noplayground
fn part_one(input: &str) -> i64 {
    // snip

    for line in lines {
        let line = line
            .replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9");

        //snip
    }

    answer
}
```

We just replace all of the instances of a written out number with a number and the parsing works! Now let's run `cargo test`. Aaaaand the test fails. With this function, we get `211` as answer, but we need `281`. Let's backtrack to the input.

It seems that our issue lies in the parsing of the text. In some lines the letters of the numbers overlap, like in `xtwone3four`. Because we convert one first, this means we get `14` instead of `24`. So, we should leave certain parts of the text intact. With a new, updated function it does work!

```rust,noplayground
fn part_one(input: &str) -> i64 {
    // snip

    for line in lines {
        let line = line
            .replace("one", "o1e")
            .replace("two", "t2o")
            .replace("three", "t3ree")
            .replace("four", "f4ur")
            .replace("five", "f5ve")
            .replace("six", "s6x")
            .replace("seven", "s7ven")
            .replace("eight", "e8ght")
            .replace("nine", "n9ne");

        //snip
    }

    answer
}
```

This does work, I've replaced every second character of the numbers with a number, so the first and last characters never interfere. The first and last are the only ones that would overlap, so this is a simple way to do it. Another way would be to convert `one` to `one1one`, which leaves the full word at the front and back intact. Now we've solved both parts for day 1, so on to day 2!

The files for this day are available [here](https://github.com/legoraft/aoc/blob/main/2023/day_01). If you want to test the full solution with the test input, check out the [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=8218e04fb1cbd290becce380c8e1ffda).