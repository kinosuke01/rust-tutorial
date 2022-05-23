## Rust Tutorial
ref: https://doc.rust-jp.rs/book-ja/
ref: https://www.rust-lang.org/ja/what/wasm
ref: https://doc.rust-jp.rs/book-ja/
ref: https://doc.rust-jp.rs/rust-by-example-ja/index.html

TODO: https://doc.rust-jp.rs/book-ja/ch12-02-reading-a-file.html

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
# ライブラリを作成する場合
cargo new adder --lib

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

# テスト
cargo test

# cargo testで使用できるオプションを表示
cargo test --help

-- という区分記号のあとに使えるオプションを表示
cargo test -- --help

# テストを実行するスレッド数を指定
cargo test -- --test-threads=1

# 成功したテストの標準出力(println!の結果など)は表示されないが、
# 以下のオプションを指定することで表示できる
cargo test -- --nocapture

# 指定文字列にマッチしたテストだけ実行する
cargo test it_works 
```

## Blocking waiting for file lock on package cache
```
cargo clean

~/.cargo/.package-cache

rm -rf ~/.cargo/registry/index/*
```

## Other
- ライブラリクレートは https://crates.io/ から検索できる 
