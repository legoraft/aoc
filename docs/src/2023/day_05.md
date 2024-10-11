# Day five

Day five gives you a list of maps and seeds, which are used to map the seeds to certain locations.

## Parsing

Parsing is done by splitting the file into blocks of maps. In my first iteration I created a separate `Block` struct, but that is unnecessary, as you can just nest the vectors.

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:76:84}}
```

The first part of the parsing just replaces all the map lines with empty lines, making parsing a bit easier. It isn't a very elegant solution, but it works as it should.

After replacing the unnecessary lines, we end up with an input which contains all the maps separated by newlines. By splitting the file once on the new line, we get the seeds. This is done in the following part:

```rust,no_run,noplayground
{{#include ../../../2023/day_05/src/main.rs:86}}
```

After this, we split the file in 

## Part one

For the first part, we need to find the lowest location number for the seeds given.

