# Day one

On day one, you are loaded in a trebuchet of which the calibration was done by a young artistic elf. Due to the artistry, the calibration value is difficult to read. We need to get the calibration value for the elves.

## Part one

For the first part, we need to find a calibration value within a line of text, grabbing only the first and last number in that line of text. These two numbers have to be concatenated and form a two digit number together. By adding up all these numbers, you find your calibration value.

I started by just creating a simple test to check if the example input equals the example answer. With this I can check if my code is correct for the example, so I can use it on my true input. Tests are really easy to write in rust, so my test looks like this:

```rust
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

```rust
fn parse(file: &str) -> Vec<&str> {
    let lines: Vec<&str> = file.lines().collect();

    lines
}
```

Now let's start with solving the problem!

```rust
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

```rust
fn main() {
    let input_file = include_str!("../../inputs/day_01.txt");

    let answer_one = part_one(input_file);

    println!("Part one: {}\n", answer_one);
}
```

Here we read the input file to a string and use this as input for our `part_one()` function. This function returns an `i64`, so we bind a variable to that and print the variable. If everything works correctly, you should see the correct answer here.