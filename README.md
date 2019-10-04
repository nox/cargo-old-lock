# `cargo-old-lock`

This crate contains a single binary that lets users print a Cargo manifest old
style, in case they experimented with the [new `Cargo.lock` format][new] and
wish to go back to the old one, for example to run a Cargo-related tool that
didn't make the switch yet.

## Usage

```
cargo run -- path/to/Cargo.lock
```

## Licensing

This crate is licensed under both the Apache 2.0 and MIT licenses, so you are
free to do whatever you want with it as long as you respect the terms from
these two.

If you are a highly paid worker at Google, Facebook, Apple, Amazon, Microsoft,
Palantir, Uber, Airbnb, Deliveroo, or any other company that prioritises profit
over people as strongly as they do, you can still use this crate. I simply wish
you will unionise and push back against the obsession for growth, control, and
power that is rampant in your workplace. Please take a stand against the
horrible working conditions they inflict on your lesser paid colleagues, and
more generally their gross disrespect for the very human rights they claim to
fight for.

[new]: https://github.com/rust-lang/rust/pull/63579
