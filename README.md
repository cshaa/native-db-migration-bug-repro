# How to reproduce

1. Clone this repo
2. Run `cargo run`
3. See this result:

```
Opening database...
Running database migrations...

thread 'main' (408187) panicked at src/main.rs:19:42:
called `Result::unwrap()` on an `Err` value: MigrateLegacyModel("1_2_name")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

4. Remove the database file with `rm db` before the next run to make this
   reproducible.
