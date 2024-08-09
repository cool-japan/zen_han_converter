# Zen Han Converter

[English/英語](README.md)

Zen Han Converterは、日本語テキストの全角文字と半角文字を相互に変換するためのRustライブラリおよびコマンドラインツールです。ASCII文字、数字、カタカナ文字の変換をサポートしています。

## 機能

- 全角文字を半角文字に変換
- 半角文字を全角文字に変換
- ASCII文字、数字、カタカナ文字のサポート
- 簡単に使用できるコマンドラインツール
- 他のRustプロジェクトに統合可能なライブラリ

## インストール

Zen Han Converterを使用するには、システムにRustがインストールされている必要があります。Rustがインストールされていない場合は、[https://www.rust-lang.org/](https://www.rust-lang.org/)からインストールできます。

その後、Cargoを使用してプロジェクトをビルドできます：

```bash
git clone https://github.com/cool-japan/zen_han_converter.git
cd zen_han_converter
cargo build --release
```

コンパイルされたバイナリは`target/release`ディレクトリ内に生成されます。

## 使用方法

### コマンドラインツール

このプロジェクトは2つのコマンドラインツールを提供しています：`zen2han`と`han2zen`。

#### zen2han

全角文字を半角文字に変換します：

```bash
echo "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５" | ./target/release/zen2han
```

#### han2zen

半角文字を全角文字に変換します：

```bash
echo "ﾊﾛｰﾜｰﾙﾄﾞ" | ./target/release/han2zen
```

両方のツールは以下のオプションをサポートしています：

- `-a`: ASCII変換を無効にする
- `-d`: 数字変換を無効にする
- `-k`: カタカナ変換を無効にする

例：

```bash
echo "ﾊﾛｰﾜｰﾙﾄﾞ123" | ./target/release/han2zen -d
```

これにより、数字以外のすべての文字が変換されます。

### ライブラリの使用

Zen Han ConverterをRustプロジェクトのライブラリとして使用するには、`Cargo.toml`に以下の依存関係を追加します：

```toml
[dependencies]
zen_han_converter = { git = "https://github.com/cool-japan/zen_han_converter.git" }
```

その後、以下のようにコード内で使用できます：

```rust
use zen_han_converter::{zen_to_han, han_to_zen};

fn main() {
    let zen_text = "ＨｅｌｌｏＷｏｒｌｄ！　１２３４５";
    let han_text = zen_to_han(zen_text, true, true, true);
    println!("半角に変換: {}", han_text);

    let zen_text = han_to_zen(&han_text, true, true, true);
    println!("全角に戻す: {}", zen_text);
}
```

## 開発貢献

コントリビューションを歓迎します！気軽にプルリクエストを送信してください。

## ライセンス

このプロジェクトはApache License 2.0の下でライセンスされています。詳細は[LICENSE](LICENSE)ファイルをご覧ください。
