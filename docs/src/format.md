This file is a format for the documentation pages on my advent of code solution. This should prevent the explanation of the solutions getting out of hand and enabling me to always write everything in the same format.

# Day {number}

{day story explanation}, preferrably continue from the last part of the story, so the storyline is as clear as possible.

## Part one

{part one explanation}, explain what the gist of the problem is, show the example input and basically try to explain what the general plan is.

#### Tests

Give an explanation for the test and why it is good to use tests

#### Parser

I try to use simple file parsers, that just split the file in lines or other sections, so most processing is done within the main functions.

#### Main function

{function explanation} explain the main `part_one()` function and explain the preliminary processing that is done. Extra headings can be added for structs, implementations etc.

#### Solution

{solution explanation} explain how the processing in the function helps to solve the problem and solve the problem

## Part two

{part two explanation} explain what is different from part one and why it can be simple/difficult.

#### Main function

{function explanation} explain what is different from the `part_one()` function and show the adjustments that should be made.

#### Solution

{solution explanation} explain what the solution is for this part and hwo processing helped.

## Conclusion

General conclusions about day (optional), also add link to github day folder and playground with working solution with test input here.

# Code formatting

Code formatting is an important part to keep the docs formatted correctly.

The function body of the function you're working in should always be visible:

```rust,ignore
fn main() {
    println!("Hello, world!");
}
```

If you want to leave some code out, make sure that the important parts are visible and use a `// --snip--` line.

```rust,ignore
fn part_one(input: &str) -> i64 {
    // --snip---
    for line in lines {
        // --snip--

        answer += number;
    }

    answer
}
```

Any other relevant lines can be included by using a `#` in front of the line. The `#rustdoc_include` syntax isn't used to keep the source code readable.

The final file is uploaded to the rust playground with the example input and the file of that day is viewable in the github repo.