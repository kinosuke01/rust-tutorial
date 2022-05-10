## Rust Tutorial
ref: https://doc.rust-jp.rs/book-ja/
ref: https://www.rust-lang.org/ja/what/wasm
ref: https://doc.rust-jp.rs/book-ja/
ref: https://doc.rust-jp.rs/rust-by-example-ja/index.html

TODO: https://doc.rust-jp.rs/book-ja/ch09-01-unrecoverable-errors-with-panic.html

## コマンドメモ
```
# インストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール場所
~/.cargo/bin

# アップデート
rustup update

# バージョン確認
rustc --version

# アンインストール
rustup self uninstall

# ドキュメント開く
rustup doc

# コンパイル
rustc main.rs

# 整形
rustfmt main.rs

# プロジェクトディレクトリ作成
cargo new hello_cargo
# バイナリ生成プロジェクトにする場合
cargo new hello_cargo --bin

# プロジェクトビルド
# target/debug/xxxx に実行ファイルができる
# 依存するライブラリのインストールも行う
cd hello_cargo
cargo build

# ライブラリのアップグレード
# Cargo.tomlで指定された範囲内での更新
cargo update

# プロジェクトビルドから実行まで一発で
cargo run

# コンパイル可能かをチェック
cargo check

# リリース用ビルド
# target/releaseに作成される
cargo build --release
```
