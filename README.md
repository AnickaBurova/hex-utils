# hex-utils
Xxd output from binary input with configurable formatting."

[Documentation](https://docs.rs/hex-utils/0.1.8/hex_utils/)

```toml
[dependencies]
hex_utils = "*"
```

Get iterator over formatted output as (offset, hex_output, ascii_output).

```rust
extern crate hex_utils;


let text = "The quick brown fox jumps over the lazy dog";

for (offset, hex, txt) in hex_utils::xxd(text.as_bytes(), None) {
    println!("offset = {:03x} hex = {:60} txt = {}", offset, hex, txt.unwrap());
}
```

```rust
extern crate hex_utils;


let text = "The quick brown fox jumps over the lazy dog";
let format = hex_utils::Format {
    size: 18,
    pack: vec![3,6],
    ascii_none: '-',
    ascii: true,
    gaps:(4,2),
};

let fmt = format.formatter();

for line in hex_utils::xxd(text.as_bytes(), Some(format)) {
    println!("{}", fmt(line));
}
```

Or one huge formatted string.
```rust
extern crate hex_utils;


let text = "The quick brown fox jumps over the lazy dog";

println!("{}", hex_utils::xxd_str(text.as_bytes(), None));
```
