// 構造体やenumをuseで持ち込むときは
// フルパスを書くのが慣習的
use std::collections::HashMap;

// モジュールのサブパスを並べて書ける
// use std::io::{
//     self,
//     Write
// };

// glob演算子でまとめて取り込める
// use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
