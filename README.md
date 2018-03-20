# lzf_headers
A thin wrapper that can compress/decompress LZF payloads, with LZF, headers for Rust

Note: If your payload does not have or does not need the headers, feel free to use the [lzf library](https://github.com/badboy/lzf-rs) by Jan-Erik Rediger.


## Usage
First, import the package. In Cargo.toml:
```toml
[dependencies]
lzf = "0.1.1"
```

Then, use the `compress` and `decompress` functions (that's the whole API, basically)

```rust
let test_string = "An adequately long string. Longer than this..."
match compress_lzf(test_string.as_bytes()) {
    Ok(decompressed) => // Handle decompressed data,
    Err(error) => // Handle error
};
```

```rust
let compressed_content = ...
match decompress_lzf(compressed_content) {
    Ok(decompressed) => println!("Decompressed: {:?}", decompressed),
    Err(error) => // Handle error
};
```

## Notes
Please let me know if you need more information, documentation, or if you found a bug.

Or, file a Pull Request, of course. ;)
