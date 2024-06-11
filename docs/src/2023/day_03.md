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

First, we need to check if any digit of a number is adjacent to a special character. We also need to get the full number to add to our answer. Because two special characters can be adjacent to the same number, I'm going to store the numbers so I don't have to deal with duplicate numbers. The code I used to find the full number and the coordinates around it is as follows.

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

The final code looks like this:

[code](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&code=use+std%3A%3Acollections%3A%3AHashSet%3B%0A%0Afn+main%28%29+%7B%0A++++let+input_file%3A+%26str+%3D+%22%5C%0A467..114..%0A...*......%0A..35..633.%0A......%23...%0A617*......%0A.....%2B.58.%0A..592.....%0A......755.%0A...%24.*....%0A.664.598..%22%3B%0A%0A++++let+answer_one+%3D+part_one%28input_file%29%3B%0A++++let+answer_two+%3D+part_two%28input_file%29%3B%0A%0A++++println%21%28%22Part+one%3A+%7B%7D%5CnPart+two%3A+%7B%7D%22%2C+answer_one%2C+answer_two%29%3B%0A%7D%0A%0A%23%5Bderive%28Debug%29%5D%0Astruct+Number+%7B%0A++++value%3A+i64%2C%0A++++coords%3A+HashSet%3C%28i64%2C+i64%29%3E%2C%0A%7D%0A%0Aimpl+Number+%7B%0A++++fn+new%28x%3A+usize%2C+y%3A+usize%2C+map%3A+%26Vec%3CVec%3Cchar%3E%3E%29+-%3E+Self+%7B%0A++++++++let+mut+number%3A+String+%3D+String%3A%3Anew%28%29%3B%0A++++++++let+mut+coords%3A+HashSet%3C%28i64%2C+i64%29%3E+%3D+HashSet%3A%3Anew%28%29%3B%0A%0A++++++++for+x+in+x..map.len%28%29+%7B%0A++++++++++++if+map%5By%5D%5Bx%5D.is_digit%2810%29+%7B%0A++++++++++++++++number.push%28map%5By%5D%5Bx%5D%29%3B%0A%0A++++++++++++++++let+x+%3D+x+as+i64%3B%0A++++++++++++++++let+y+%3D+y+as+i64%3B%0A%0A++++++++++++++++coords.extend%28HashSet%3A%3Afrom%28%5B%0A++++++++++++++++++++%28x+-+1%2C+y+-+1%29%2C+%28x+-+1%2C+y%29%2C+%28x+-+1%2C+y+%2B+1%29%2C%0A++++++++++++++++++++%28x%2C+y+-+1%29%2C+%28x%2C+y+%2B+1%29%2C%0A++++++++++++++++++++%28x+%2B+1%2C+y+-+1%29%2C+%28x+%2B+1%2C+y%29%2C+%28x+%2B+1%2C+y+%2B+1%29%2C%0A++++++++++++++++%5D%29%29%3B%0A++++++++++++%7D+else+%7B%0A++++++++++++++++break%3B%0A++++++++++++%7D%0A++++++++%7D%0A%0A++++++++Number+%7B%0A++++++++++++value%3A+number.parse%3A%3A%3Ci64%3E%28%29.expect%28%22Couldn%27t+parse+number%21%22%29%2C%0A++++++++++++coords%2C%0A++++++++%7D%0A++++%7D%0A%7D%0A%0Afn+part_one%28input%3A+%26str%29+-%3E+i64+%7B%0A++++let+map+%3D+parser%28input%29%3B%0A++++let+mut+answer+%3D+0%3B%0A%0A++++let+mut+numbers%3A+Vec%3CNumber%3E+%3D+Vec%3A%3Anew%28%29%3B%0A++++let+mut+symbols%3A+HashSet%3C%28i64%2C+i64%29%3E+%3D+HashSet%3A%3Anew%28%29%3B%0A%0A++++for+%28y%2C+line%29+in+map.iter%28%29.enumerate%28%29+%7B%0A++++++++let+mut+n+%3D+0%3B%0A%0A++++++++for+%28x%2C+%26ch%29+in+line.iter%28%29.enumerate%28%29+%7B%0A++++++++++++if+n+%3E+0+%7B%0A++++++++++++++++n+-%3D+1%3B%0A++++++++++++++++continue%3B%0A++++++++++++%7D%0A++++++++++++%0A++++++++++++if+ch.is_digit%2810%29+%7B%0A++++++++++++++++let+num+%3D+Number%3A%3Anew%28x%2C+y%2C+%26map%29%3B%0A++++++++++++++++n+%2B%3D+%28num.value.to_string%28%29%29.len%28%29+-+1%3B%0A++++++++++++++++numbers.push%28num%29%3B%0A++++++++++++%7D+else+if+ch+%21%3D+%27.%27+%7B%0A++++++++++++++++let+coords+%3D+%5B%0A++++++++++++++++++++%28x+as+i64%2C+y+as+i64%29%2C%0A++++++++++++++++%5D%3B%0A++++++++++++++++symbols.extend%28coords%29%3B%0A++++++++++++%7D+else+%7B%0A++++++++++++++++continue+%0A++++++++++++%7D%0A++++++++%7D%0A++++%7D%0A%0A++++for+number+in+numbers+%7B%0A++++++++if+number.coords.intersection%28%26symbols%29.next%28%29.is_some%28%29+%7B%0A++++++++++++answer+%2B%3D+number.value%3B%0A++++++++%7D%0A++++%7D%0A++++%0A++++answer%0A%7D%0A%0Afn+part_two%28input%3A+%26str%29+-%3E+i64+%7B%0A++++let+map+%3D+parser%28input%29%3B%0A++++let+mut+answer+%3D+0%3B%0A%0A++++let+mut+numbers%3A+Vec%3CNumber%3E+%3D+Vec%3A%3Anew%28%29%3B%0A++++let+mut+gears%3A+HashSet%3C%28i64%2C+i64%29%3E+%3D+HashSet%3A%3Anew%28%29%3B%0A%0A++++for+%28y%2C+line%29+in+map.iter%28%29.enumerate%28%29+%7B%0A++++++++let+mut+n+%3D+0%3B%0A%0A++++++++for+%28x%2C+%26ch%29+in+line.iter%28%29.enumerate%28%29+%7B%0A++++++++++++if+n+%3E+0+%7B%0A++++++++++++++++n+-%3D+1%3B%0A++++++++++++++++continue%3B%0A++++++++++++%7D%0A++++++++++++%0A++++++++++++if+ch.is_digit%2810%29+%7B%0A++++++++++++++++let+num+%3D+Number%3A%3Anew%28x%2C+y%2C+%26map%29%3B%0A++++++++++++++++n+%2B%3D+%28num.value.to_string%28%29%29.len%28%29+-+1%3B%0A++++++++++++++++numbers.push%28num%29%3B%0A++++++++++++%7D+else+if+ch+%3D%3D+%27*%27+%7B%0A++++++++++++++++let+coords+%3D+%5B%0A++++++++++++++++++++%28x+as+i64%2C+y+as+i64%29%2C%0A++++++++++++++++%5D%3B%0A++++++++++++++++gears.extend%28coords%29%3B%0A++++++++++++%7D+else+%7B%0A++++++++++++++++continue+%0A++++++++++++%7D%0A++++++++%7D%0A++++%7D%0A%0A++++for+gear+in+gears+%7B%0A++++++++let+mut+hits%3A+Vec%3Ci64%3E+%3D+Vec%3A%3Anew%28%29%3B+%0A++++++++for+number+in+%26numbers+%7B%0A++++++++++++if+number.coords.contains%28%26gear%29+%7B%0A++++++++++++++++hits.push%28number.value%29%3B%0A++++++++++++%7D%0A++++++++%7D%0A++++++++if+hits.len%28%29+%3D%3D+2+%7B%0A++++++++++++answer+%2B%3D+hits%5B0%5D+*+hits%5B1%5D%3B%0A++++++++%7D%0A++++%7D%0A++++%0A++++answer%0A%7D%0A%0Afn+parser%28file%3A+%26str%29+-%3E+Vec%3CVec%3Cchar%3E%3E+%7B%0A++++let+map%3A+Vec%3CVec%3Cchar%3E%3E+%3D+file%0A++++++++.lines%28%29%0A++++++++.map%28%7Cs%7C+s.chars%28%29.collect%28%29%29%0A++++++++.collect%28%29%3B%0A%0A++++map%0A%7D)