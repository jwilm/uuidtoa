uuidtoa
=======

Optimized serialization of hyphenated UUID ASCII representation to `io::Write`
sinks.

The two functions exported by this crate are `write_lower` and `write_upper`;
here are their signatures:

```rust
fn write_lower<W: io::Write>(io: &mut W, uuid: &Uuid) -> io::Result<usize>;
fn write_upper<W: io::Write>(io: &mut W, uuid: &Uuid) -> io::Result<usize>;
```

An example of writing this out to a buffer would be

```rust
let uuid = Uuid::from_str("e098d1f6-7e49-4c34-99f6-e9c6cef8fcce").unwrap();
let mut buf: Vec<u8> = Vec::with_capacity(36);
write_lower(&mut buf, &uuid).unwrap();
```

This performs approximately 10x better than the equivalent `fmt::Display` impl
from the _uuid_ crate.

```rust
let uuid = Uuid::from_str("e098d1f6-7e49-4c34-99f6-e9c6cef8fcce").unwrap();
let buf = format!("{}", uuid.hyphenated());
```

The benchmarks from these two approaches are as follows.

```
test benches::bench_fmt   ... bench:         522 ns/iter (+/- 6)
test benches::bench_lower ... bench:          45 ns/iter (+/- 0)
```

To run them for yourself, the bench feature and nightly compiler should be used.

```sh
cargo bench --feature bench
```
