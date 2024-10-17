# Day seven

On day 7 we try and implement a simplified poker analyzer.

## Parsing

The parser for this day isn't that complicated. It has been implemented as the following:

```rust,no_run,noplayground
{{#include ../../../2023/day_07/src/main.rs:153:154}}
```

At first, we get all the lines and use `.map()` to apply some logic to every line. We split the lines at the space once, which separated the cards and the bids. This gives us a tuple, and by collecting this we end up with a vec of tuples.

Now we need to create cards. All the cards are made up of characters, but we can't sort them easily. Because of this we've created a cards enum with an implementation that enables us to sort the cards.

```rust,no_run,noplayground
{{#include ../../../2023/day_07/src/main.rs:19:56}}
```

This matches all the characters and assigns the correct cards to it. It aso contains a `joker` boolean, which we'll use in part two.

To get all the data in a better format, we created a `Hand` struct with the cards, score and the bid. The score will help us determining the hand rank soon.

```rust,no_run,noplayground
{{#include ../../../2023/day_07/src/main.rs:12:17}}
```

We parse all the data in the following function:

```rust,no_run,noplayground
{{#include ../../../2023/day_07/src/main.rs:156:168}}
```

For every hand, we parse the bid to an `i64`, all the cards get split into `.chars()` and gets parsed into a `Card` type. After this, we push the `Hand` struct to the games vec. We set the score to 0 to modify later.

## Part one

In part one, we just need to build a simple poker parser.
