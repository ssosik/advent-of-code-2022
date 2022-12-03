# advent-of-code-2022

My solutions to the 2022 AoC as I work through them. I'm using Rust and
[cargo-aoc](https://github.com/gobanos/cargo-aoc) to get the inputs and run the
code. Read the
[aoc_runner_derive](https://docs.rs/aoc-runner-derive/0.3.0/aoc_runner_derive/index.html)
macro docs for more details on how to hook into it in the code.

With cargo aoc installed, running the solutions is as simple as
```bash
# Run the most recent day
❯ cargo aoc
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
   Compiling aoc-autobuild v0.3.0 (/Users/stevesosik/workspace/advent-of-code-2022/target/aoc/aoc-autobuild)
    Finished release [optimized] target(s) in 0.15s
     Running `target/release/aoc-autobuild`
AOC 2022
Day 2 - Part 1 : 15337
	generator: 250ns,
	runner: 1.112375ms

Day 2 - Part 2 : 11696
	generator: 125ns,
	runner: 1.046208ms

# Or run a specific day
❯ cargo aoc -d 1
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
   Compiling aoc-autobuild v0.3.0 (/Users/stevesosik/workspace/advent-of-code-2022/target/aoc/aoc-autobuild)
    Finished release [optimized] target(s) in 0.15s
     Running `target/release/aoc-autobuild`
AOC 2022
Day 1 - Part 1 : 74711
	generator: 169.125µs,
	runner: 500ns

Day 1 - Part 2 : 209481
	generator: 123.708µs,
	runner: 1.75µs
```

To add a new day, create the corresponding day file, e.g. `day3.rs`, stub out
the solution methods and update the macro calls:
```rust
#[aoc(day2, part1)]
pub fn part1(input: &str) -> i32 {
    ...
}
```

then call `cargo aoc` to fetch the test inputs and run the code.
