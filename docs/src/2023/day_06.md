# Day six

On day six we start racing with boats. This is done by pressing a button and releasing it, making the boat travel at a certain speed.

## Parsing

Parsing was quite easy for this day. You just have two lines you need to parse. We start by getting the lines and parsing per line. We do this as follows:

```rust,no_run,noplayground
{{#include ../../../2023/day_06/src/main.rs:59:63}}
```

In this, we start by dropping the first part of the line, which is separated by a colon. After this, we end up with a string of numbers separated by spaces.

We do this for both lines and parse the numbers into a vec of `i64`. Finally, we create the races as a vec of tuples.

```rust,no_run,noplayground
{{#include ../../../2023/day_06/src/main.rs:67:69}}
```

This iterates over the times and keeps track of the index with `.enumerate()`. After that, we just push the time and the record at the same index as a tuple to the races vector.

## Part one

For part one we need to multiply all the possibilities per race. The fastest way to calculate the possibilities to break the records is to go up to the first broken record and multiplying this by 2. We also need to add one because otherwise every answer will fall 1 short.

This full calculation is done in the following loop:

```rust,no_run,noplayground
{{#include ../../../2023/day_06/src/main.rs:14:24}}
```

Here we check every time from 0 to the max time. After this we calculate the distance traveled with `distance = (time - press) * press`. We check if the distance traveled is larger than the record, and if that is so we calculate the possibilities with `(time - press * 2) + 1`. After that, we multiply our current answer with the amount of possibilities and break out of the loop.

## Part two

The next part is mostly a parsing challenge. We need to combine all the time numbers and the record numbers for one big race.

We can do this by iterating over evry number, converting it to a string and pushing it to a single string. This is done in the following snippet:

```rust,no_run,noplayground
{{#include ../../../2023/day_06/src/main.rs:32:38}}
```

After that, we parse the time and record and just use the same algorithm as in the first part.

## Conclusion


