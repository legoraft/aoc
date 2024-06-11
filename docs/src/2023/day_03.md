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

This function splits the input into lines and maps the characters in every line to a vector of characters. This makes it so that we have a 2 dimensional vector of all the characters in the map Now we need to find out if any number is adjacent to a special character. This problem is quite difficult to solve, so let's take a look at the conditions we need to add a number.

First, we need to check if any digit of a number is adjacent to a special character. We also need to get the full number to add to our answer. Because two special characters can be adjacent to the same number, I'm going to store the numbers so I don't have to deal with duplicate numbers. Let's try to find a digit in our array of characters first.

```rust,noplayground
let mut numbers: Vec<Number> = Vec::new();
let mut symbols: HashSet<(i64, i64)> = HashSet::new();

for (y, line) in map.iter().enumerate() {
    let mut n = 0;

    for (x, &ch) in line.iter().enumerate() {
        if n > 0 {
            n -= 1;
            continue;
        }
        
        if ch.is_digit(10) {
            let num = Number::new(x, y, &map);
            n += (num.value.to_string()).len() - 1;
            numbers.push(num);
        } else if ch != '.' {
            let coords = [
                (x as i64, y as i64),
            ];
            symbols.extend(coords);
        } else {
            continue 
        }
    }
}
```

The code I used to find the full number and the coordinates around it is as follows.

```rust,noplayground
struct Number {
    value: i64,
    coords: HashSet<(i64, i64)>,
}

impl Number {
    fn new(x: usize, y: usize, map: &Vec<Vec<char>>) -> Self {
        let mut number: String = String::new();
        let mut coords: HashSet<(i64, i64)> = HashSet::new();

        for x in x..map.len() {
            if map[y][x].is_digit(10) {
                number.push(map[y][x]);

                let x = x as i64;
                let y = y as i64;

                coords.extend(HashSet::from([
                    (x - 1, y - 1), (x - 1, y), (x - 1, y + 1),
                    (x, y - 1), (x, y + 1),
                    (x + 1, y - 1), (x + 1, y), (x + 1, y + 1),
                ]));
            } else {
                break;
            }
        }

        Number {
            value: number.parse::<i64>().expect("Couldn't parse number!"),
            coords,
        }
    }
}
```

The pseudocode I used:

```rust,noplayground
/*
    
let map = parser(input);

for (y, line) in map.iter.enumerate() {
    for (x, ch) in line.iter.enumerate() {
        if ch == digit {
            let (number, coords) = extend_digit()
            nums.push(Number {
                number,
                coords
            })
        } if ch == symbol {
            let coords = get_coords()
            syms.push(coords)
        } else {
            continue 
        }
    }
}

if syms.contains(nums.number.coords) {
    answer += number
}

extend_digit() {
    let mut number = vec!
    let mut coords = HashSet
    
    for x in x..map.len() {
        if x.isdigit() {
            number.push map[y][x]
            coords.insert((x, y))
        } else {
            break 
        }
    }
    number.parse, coords
}
*/
```

To run the code with the test input, check out the [playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=69fc7faa79dd1f893791158cea54f142).