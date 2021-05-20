# time-iso8601-serde

Serialize/deserialize [time](https://github.com/time-rs/time) to/from ISO8601 formatted string, using [iso8601](https://github.com/badboy/iso8601).

NOTE: Not published to crates.io (yet)

```rust
#[derive(Deserialize, Serialize)]
struct SomeEntity {
    #[serde(with = "time_iso8601_serde::datetime")]
    created_at: time::OffsetDateTime,
}
```

## License

This project is licensed under either of

- [Apache License, Version 2.0](LICENSE-Apache)
- [MIT license](LICENSE-MIT)

at your option.
