fn main() {
    // 温度を華氏と摂氏で変換する。
    println!("{}", to_fahrenheit(10.0));
    println!("{}", to_celsius(50.0));

    // フィボナッチ数列のn番目を生成する。
    println!("{}", fibonacci_of(10));

    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("{}", get_mean(v));

    let v = vec![15, 30, 25, 100, 45, 10, 70, 96, 60];
    println!("{}", get_dian(v));
    // println!("{}", get_mode(v));
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
fn get_dian(list: Vec<i32>) -> i32 {
    list.sort();
    let i = list.len() / 2;
    list[i] as i32
}
