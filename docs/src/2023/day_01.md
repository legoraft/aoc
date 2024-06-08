# Day one

On day one, you are loaded in a trebuchet of which the calibration was done by a young artistic elf.

## Part one

For the first part, you need to find a calibration value within a line of text, grabbing only the first and last number in that line of text. These two numbers have to be concatenated and form a two digit number together. By adding up all these numbers, you find your calibration value.

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

To get every digit from 