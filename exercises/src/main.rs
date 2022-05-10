use std::collections::HashMap;
use rand::Rng;

fn main() {
    // 温度を華氏と摂氏で変換する。
    println!("{}", to_fahrenheit(10.0));
    println!("{}", to_celsius(50.0));

    // フィボナッチ数列のn番目を生成する。
    println!("{}", fibonacci_of(10));

    let v = build_rand_int_list(10, 10);
    println!("mean of is {}", get_mean(v));

    let v = build_rand_int_list(10, 10);
    println!("dian is {}", get_dian(v));

    let v = build_rand_int_list(10, 1000);
    println!("{}", get_mode(v));
}

fn to_fahrenheit(x: f64) -> f64 {
    x * 1.8 + 32.0
}

fn to_celsius(x: f64) -> f64 {
    (x - 32.0) / 1.8
}

fn fibonacci_of(x: u32) -> u32 {
    if x == 0 || x == 1{
        x
    } else {
        fibonacci_of(x - 2) + fibonacci_of(x - 1)
    }
}

// ランダムな整数を要素とする配列を作る
fn build_rand_int_list(max: i32, size: i32) -> Vec<i32> {
    let mut list = vec![];
    let mut i = 0;
    while i < size {
        let number = rand::thread_rng().gen_range(1..(max + 1));
        list.push(number);
        i += 1;
    }
    list
}

// 平均値
fn get_mean(list: Vec<i32>) -> f32 {
    let mut sum = 0;
    for value in &list {
        sum += value;
    }
    let sum = sum as f32;
    let len = list.len() as f32;
    sum / len
}

// 中央値
fn get_dian(mut list: Vec<i32>) -> i32 {
    list.sort();
    let i = list.len() / 2;
    list[i]
}

// 最頻出の整数を取り出す
fn get_mode(list: Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    let mut max_count = 0;
    let mut max_number: i32 = 0;

    for number in &list {
        let count = map.entry(number).or_insert(0);
        *count += 1;
        if max_count < *count {
            max_count = *count;
            max_number = *number;
        }
    }
    max_number
}
