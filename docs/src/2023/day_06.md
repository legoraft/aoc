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
