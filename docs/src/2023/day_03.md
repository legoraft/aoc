# Day three

For day three, we need to find a certain engine part for an elf engineer so we can go up a gondola lift. We need to find this part number within an engine schematic.

## Part one

For part one, we need to add up all part numbers that are adjecent to a symbol. These numbers can be adjacent horizontal, vertical or diagonal. This is what makes the challenge a bit difficult.

My first idea is to create a vector of every line, with every character in the line being stored in a vector. With this we can have a coordinate system, with the index in vector 1 being y and the index in vec 2 being x. This can be whipped up with a simple parser function.

```rust, noplayground
fn parser(file: &str) -> Vec<Vec<char>> {
    let map: Vec<Vec<char>> = file
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    map
}
```

This function splits the input into lines and maps the characters in every line to a vector of characters. Now we need to find out if any number is adjacent to a special character. 