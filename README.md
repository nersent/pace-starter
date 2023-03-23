# **PACE** starter

This repository is a _MACD_ boilerplate project in Rust for [Pace](https://github.com/nersent.pace), a technical analysis library.

## Getting started

1. Download [Rust and Cargo](https://doc.rust-lang.org/book/ch01-01-installation.html)

2. Install [cargo binstall](https://github.com/cargo-bins/cargo-binstall)

3. Install [cargo nextest](https://github.com/nextest-rs/nextest)

   - https://nexte.st/book/pre-built-binaries

   ```bash
   cargo binstall cargo-nextest
   ```

4. Install [cargo watch](https://github.com/watchexec/cargo-watch)

   ```bash
   cargo binstall cargo-watch
   ```

## Development

Run with hot reload

```bash
cargo watch -x run
```

## Examples

See already implemented indicators and strategies [here](https://github.com/nersent/pace/pace/src/content).

## Testing

```bash
cargo nextest run
```

This repository contains a simple test for macd indicator, tested against TradingView MACD indicator.

### Test against PineScript

1. You need to add this script at the end of PineScript:

   ```pine
   plot(PS_RESULT, title='_target_')
   plot(volume, title='volume')
   ```

2. Export to csv

   - Open the downloaded csv file and remove entire last row with `NaN` values (if exists)

3. Place the csv file in `fixtures` folder

4. Modify `src/tests/custom_indicator_test.rs`

## Structure

The files you should look at are:

- `src/custom_strategy.rs`
- `src/custom_indicator.rs`
- `src/tests/custom_indicator_test.rs`

The recommendation is to seperate your strategy and indicator into separate modules, allowing easier testing and development.

## Documentation

Visit [docs](https://github.com/nersent.pace/docs/index.md) to view Pace documentation.

---

Made by [Nersent](https://nersent.com)
