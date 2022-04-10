fn main() {
    println!("Hello, world!");

    tup_fn();
    array_fn(3);
    array_fn(14);
    println!("{}", plus_one(10));
    if_fn();
    while_fn();
    for_fn();

    // 温度を華氏と摂氏で変換する。
    println!("{}", to_fahrenheit(10.0));
    println!("{}", to_celsius(50.0));

    // フィボナッチ数列のn番目を生成する。
    println!("{}", fibonacci_of(10));
}

fn tup_fn() {
    // タプル型
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x,y,x is {}, {}, {}", x, y, z);
}

// usizeはCPUアーキテクチャに沿ったサイズの正の整数型
// 32bit -> u32
// 64bit -> u64
fn array_fn(i: usize) {
    // 配列 - 要素数は固定
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    if i < months.len() {
        println!("months[{}] is {}", i, months[i]);
    } else {
        println!("{} is invalid value.", i);
    }
}

fn plus_one(x: i32) -> i32 {
    // 文は値を返さない
    // 最後にセミコロンを付ける
    let val = x + 1;

    // 式は値を返す
    // 最後にセミコロンを付けない
    // return val; でも OK
    val
}

fn if_fn() {
    let number = 6;
    if number % 4 == 0 {
        println!("number is devisible by 4");
    } else if number % 3 == 0 {
        println!("number is deivisible by 3");
    } else {
        println!("number is not dividible by 4, 3");
    }

    let condition = true;
    let number = if condition {
        5 // 式
    } else {
        6 // 式
    };
    println!("The value of number is: {}", number);
}

fn while_fn() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }

    // 発射!!
    println!("LIFTOFF!!!");
}

fn for_fn() {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 逆順にしてループ
    // (1..4)は末尾の4を含まない
    for number in (1..4).rev() {
        println!("{}", number);
    }
    println!("LIFTOFF!!!!!");
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
