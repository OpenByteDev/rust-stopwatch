<a href="https://crates.io/crates/stopwatch2">
    <img src="https://img.shields.io/crates/v/stopwatch2.svg" alt="Stopwatch2" />
</a>

# Stopwatch V2
Support an Open Source Developer! :hearts:  
[![Become a patron](https://c5.patreon.com/external/logo/become_a_patron_button.png)](https://www.patreon.com/jojolepro)

Read the [documentation](https://docs.rs/stopwatch2).

This crate is a simplified version of [the work of ellisonch](https://github.com/ellisonch/rust-stopwatch).

# Features

* Adds a simple stopwatch.
* Can use multiple splits, even with pauses between them!
* Simple to use with clear documentation.

# Usage
Add the following to you Cargo.toml file:

```
stopwatch2 = "*"
```

Use the stopwatch like so:
```rust
use stopwatch2::*;

fn main() {
    let mut s = Stopwatch::default();
    s.start(); // Starts the stopwatch.
    s.start(); // Creates a new time span, which are commonly called "splits".
    s.stop(); // Stops the stopwatch.
    println!("{}", s); // Prints the total time.
    println!("{:?}", s); // Prints the different time spans as debug information.
    let total_time = s.elapsed(); // returns the total time as a Duration.
    for span in &s.spans {
        // Prints all contained time spans.
        println!("{:?} -> {:?}", span.start, span.stop);
    }
    s.spans.clear(); // Reset the stopwatch.
    println!("{}", s); // Prints the total time.
    println!("{:?}", s); // Prints the different time spans as debug information.
}
```
### Maintainer Information

* Maintainer: Jojolepro
* Contact: jojolepro [at] jojolepro [dot] com
* Website: [jojolepro.com](https://jojolepro.com)
* Patreon: [patreon](https://patreon.com/jojolepro)

