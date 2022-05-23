// 結合テストは、testsディレクトリ配下に書く
// このファイルのみテスト実行するときは
// cargo test --test integration_test

// src/main.rsファイルのみを含み、src/lib.rsファイルを持たないバイナリクレートだったら、
// testsディレクトリに結合テストを作成し、 
// extern crateを使用してsrc/main.rsファイルに定義された関数をインポートすることはできない。

// testsはテスト対象と別のクレートであるためインポートが必要
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::it_adds_two(2));
}
