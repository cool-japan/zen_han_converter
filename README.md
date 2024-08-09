# Zen Han Converter

[Japanese/日本語](README.ja.md)

Zen Han Converter is a Rust library and set of command-line tools for converting between Zenkaku (full-width) and Hankaku (half-width) characters in Japanese text. It supports conversion of ASCII characters, digits, and Kana characters.

## Features

- Convert Zenkaku characters to Hankaku
- Convert Hankaku characters to Zenkaku
- Supports ASCII characters, digits, and Kana characters
- Command-line tools for easy use
- Library for integration into other Rust projects

## Installation

To use Zen Han Converter, you need to have Rust installed on your system. If you don't have Rust installed, you can install it from [https://www.rust-lang.org/](https://www.rust-lang.org/).

Then, you can build the project using Cargo:

```bash
git clone https://github.com/cool-japan/zen_han_converter.git
cd zen_han_converter
cargo build --release
```

The compiled binaries will be available in the `target/release` directory.

## Usage

### Command-line Tools

This project provides two command-line tools: `zen2han` and `han2zen`.

#### zen2han

Converts Zenkaku characters to Hankaku:

```bash
echo "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５" | ./target/release/zen2han
```

#### han2zen

Converts Hankaku characters to Zenkaku:

```bash
echo "ﾊﾛｰﾜｰﾙﾄﾞ" | ./target/release/han2zen
```

Both tools support the following options:

- `-a`: Disable ASCII conversion
- `-d`: Disable digit conversion
- `-k`: Disable Kana conversion

Example:

```bash
echo "ﾊﾛｰﾜｰﾙﾄﾞ123" | ./target/release/han2zen -d
```

This will convert all characters except the digits.

### Library Usage

To use Zen Han Converter as a library in your Rust project, add it as a dependency in your `Cargo.toml`:

```toml
[dependencies]
zen_han_converter = { git = "https://github.com/cool-japan/zen_han_converter.git" }
```

Then you can use it in your code:

```rust
use zen_han_converter::{zen_to_han, han_to_zen};

fn main() {
    let zen_text = "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５";
    let han_text = zen_to_han(zen_text, true, true, true);
    println!("Converted to Hankaku: {}", han_text);

    let zen_text = han_to_zen(&han_text, true, true, true);
    println!("Converted back to Zenkaku: {}", zen_text);
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the ASL 2.0 License - see the [LICENSE](LICENSE) file for details.
