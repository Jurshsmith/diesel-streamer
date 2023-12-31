# Diesel Streamer

[<img alt="github" src="https://img.shields.io/badge/Github-jurshsmith%2Fdiesel--streamer-blue?logo=github" height="20">](https://github.com/jurshsmith/diesel-streamer)
[<img alt="crates.io" src="https://img.shields.io/crates/v/diesel-streamer.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/diesel-streamer)
[<img alt="diesel-streamer build" src="https://img.shields.io/github/actions/workflow/status/jurshsmith/diesel-streamer/ci.yml?branch=main&style=for-the-badge" height="20">](https://github.com/jurshsmith/diesel-streamer/actions?query=branch%3Amain)

A tiny diesel add-on for streaming large tables. It currently allows streaming
large serial tables using a cursor-based streaming strategy.

## Installation

For a regular synchronous runtime:

```toml
[dependencies]
diesel-streamer = { version = "0.1.12", features = ["sync"]}
```

For tokio async runtime:

```toml
[dependencies]
diesel-streamer = { version = "0.1.12", features = ["async"]}
```

<br>

## Example Usage

Stream `SomeTable` that has a `serial_field`:

```rust
use diesel_streamer::stream_serial_table;

fn main() {
  use crate::schema::some_table::dsl::{some_table, serial_field};

  let mut conn = pool.get().await.unwrap();

  // with default chunk size of 500
  stream_serial_table!(some_table, serial_field, conn, |streamed_table_data: Vec<SomeTable>| {
    // do work here
    dbg!(streamed_table_data);
  });

  // specify chunk size, 130
  stream_serial_table!(some_table, serial_field, conn, 130, |streamed_table_data: Vec<SomeTable>| {
    // do work here
    dbg!(streamed_table_data);
  });

  // with cursor's beginning, 5.
  stream_serial_table!(some_table, serial_field, conn, 130, 5, |streamed_table_data: Vec<SomeTable>| {
    // do work here
    dbg!(streamed_table_data);
  });

  // with cursor's end, 50,
  stream_serial_table!(some_table, serial_field, conn, 130, 5, 50, |streamed_table_data: Vec<SomeTable>| {
    // do work here
    dbg!(streamed_table_data);
  });
}
```

Defaults:

- Chunk size: 500
- Cursor's beginning: lowest value of `serial_field` in the table
- Cursor's end: highest value of `serial_field` in the table

_N/B: Generally, streaming should only be considered when there is a possibility of hitting OOM error when processing the table in question._

## Contributing

Spin up a test db using `docker-compose up` or simply specify
a DB url in `.env` as shown in `.env.sample`.

Run `cargo test` for tests.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
