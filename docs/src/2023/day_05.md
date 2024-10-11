# Day five

Day five gives you a list of maps and seeds, which are used to map the seeds to certain locations.

## Parsing

Parsing is done by splitting the file into blocks of maps. In my first iteration I created a separate `Block` struct, but that is unnecessary, as you can just nest the vectors.

The first part of the parsing just replaces unnecessary text and splits of the seeds by splitting once on the first newline. This works because the seeds are separated by a newline.

Next, we parse the seeds into a vector, which makes them easier to use. After this, the maps need to be separated into vectors. This is done in the following line:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:92}}
```

This line does a lot of things, so let's split it up. First, the remaining lines (without the seeds) are splitted at every newline. Because every map block is separated by the newline we end with the map blocks. This is done by the `.split("\n\n")`. Next we separate the map blocks by using the `.map(|m| m.lines()`. We still can have some empty lines though, so we use `.filter(|l| l != &"")`. After this, we just plug some `.collect()` calls to collect them into a `Vec<Vec<&str>`.

After this, we just iterate over every block. Within the block we have separate lines, which we need to put into a `Map` struct. The struct looks as follows:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:10:15}}
```

It just gives names to every part of a single map line, so we can use them more easily in the program. After this we have nested iteration to map everything:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:97:111}}
```

We iterate over every block, after which we separate every line to create a `Map`. All the map values are split at the whitespace and collected into a `Vec<i64>`. This `Map` gets pushed to a `Vec`, which gets pushed into a `blocks` vec after every block. This gives us a `Vec<Vec<Map>>`. We can iterate over the inside part of the `blocks` variable to get every map value.

## Part one

For the first part, we need to find the lowest location number for the seeds given. After parsing the file in a bit of a simpler data structure, we can solve for part one.

We just create a vector for the positions, in which we can find the lowest number easily.

Now we iterate over every seed and apply all the necessary transforms. This is done easily with the following code block:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:22:33}}
```

We iterate over every seed and iterate over every block of maps within the seed. After that, we iterate over all the map lines to check if the seed falls within the source range.

We can find the next seed value by using the following equation: `seed = seed + (map.destination - map.source)`. This is what we do if a seed falls within the source range of a map line. After this we break out of the block, because the seed can be counted within the same map block, but that can't be done due to the rules.

After we've done all these transformations, we push the final location to the `positions` vec. After we've iterated over every seed, we can use the following to find the smallest location.

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:35:39}}
```

This iterates over every position and finds the smallest value. This is finally returned.

## Part two

For part two, it seems that the seeds are actually ranges of seeds. We need to find the smallest location again.

Because iterating over every range of seeds would be insane and take too much time, we need to find a different approach. The seeds we would need to iterate over would be many millions, but the smallest seed would probably be a few million.

This means we can iterate over the final location and just reverse search for the seed. To start with this, we need to get all the seed ranges and reverse the blocks to reverse search for the seed.

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:46:50}}
```

We first split the seeds into ranges and seeds. This is done by stepping every 2 items, so we get every odd item for the ranges. This is done with the `.step_by(2)` function. We do the same thing for the ranges, but skip the first item and get every even item for the seeds.

Next, we create a vector of tuples with `Vec<(&i64, &i64)>`. Within this, the first number is the seed and the second is the range. This is done by using `.zip()`. The item that's used as an argument, will be the second item in the tuple. After collecting this we have a vector of tuples.

After this, we iterate over every location from `0`.

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:52:70}}
```

This starts by setting a seed by using `i`. After this, we iterate over all the blocks and reverse the equation used for converting from a seed to a location. This is done by using `seed = &seed - (map.destination - map.source)` when a destination range contains the seed. After this we still need to break, to prevent converting double.

After we've gotten the final seed, we need to check if it falls within the seed sources. This is done as follows:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:64:69}}
```

We create a range of numbers from the source to the source and the range added up. After this we can check if the range contains our found seed and if this is true, we return `i`. This function can run indefinitely, but because the program needs to find an answer, this isn't as bad.

## Conclusion

The most difficult part of this day was definitely the second part. The algorithm I've used also isn't the most efficient, but finds the answer in 6 seconds which is relatively good.

Parsing was a bit of a challenge this day, but after simplifying it a bit, solving the parts became a lot easier.
