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

    // 所有権について
    ownership_fn();

    // 参照渡し
    ref_fn();

    // 文字列スライス
    slice_fn();
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

// 所有権
// ref: https://doc.rust-jp.rs/book-ja/ch04-00-understanding-ownership.html
fn ownership_fn() {
    // sがスコープに入ると、有効になる
    // スコープを抜けるまで、有効なまま
    let s = "hello";
    println!("{} s!!", s);

    let mut s = String::from("hello"); // 可変データをヒープにメモリを確保
    s.push_str(", world!");            // リテラルをstringに追加
    println!("{}", s);

    let s1 = String::from("hello"); // 可変データをヒープにメモリ確保
    let s2 = s1;                    // 実データの所有権がs2にムーブ
    // println!("{}, world!", s1);  // ここではs1はもう使えない
    println!("{}, move world!", s2);

    let s1 = String::from("hello");
    let s2 = s1.clone();  // 実データ含めてコピー
    println!("s1 = {}, s2 = {}, deep copy world!", s1, s2);

    let x = 5; // リテラルは不変長なのでスタックにメモリ確保
    let y = x; // スタックのデータは実データごとコピー
    println!("x = {}, y = {}", x, y);

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    }
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s = {}", s); // 関数に所有権がムーブしたので使えない

    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
    let x = 5;
    makes_copy(x);
    println!("x = {}", x); // リテラルはコピーされるので関数実行後も使える

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
    let s1 = String::from("takes_and_gives_tack string");
    let s2 = takes_and_gives_back(s1); // 所有権が関数から呼び出しもとにムーブ
    println!("{}", s2);                // 呼び出し元にむーぶされたので使える
}

fn ref_fn() {
    // &Stringで参照渡しになる
    // 所有権は関数呼び出し側のまま、この関数が変数を借用する
    fn calculate_length(s: &String) -> usize {
        s.len()
    } // ここでsのスコープは外れるが破棄はされない

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len); // s1は破棄されないのでここでも使える

    // &mutとすることで可変となる
    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }
    let mut s = String::from("hello");
    change(&mut s);
    println!("s is {}", s);

    // 複数の可変な参照は持てない
    /*
    let mut s = String::from("hello");
    let r1 = &mut s;
    let r2 = &mut s;
    */

    // 複数の不変な参照は持てるけど、
    // 不変な参照を作成したら可変な参照は作成できない
    /*
    let mut s = String::from("hello");
    let r1 = &s;     // OK
    let r2 = &s;     // OK
    let r3 = &mut s; // NG
    */

    // 関数を抜けたときに変数の実体がムーブされる
    // 参照を返したとしても、実データがないためエラーとなる
    /*
    fn dangle() -> &String {
        let s = String::from("hello");
        &s
    }
    */
}

fn slice_fn() {
    // 文字列スライス
    // 文字列への部分的な参照
    let s = String::from("hello world");
    let hello = &s[0..5];  // 0から4(5の1つ手前)の文字列
    let world = &s[6..11]; // 6から10(11の1つ手前)の文字列
    let hello2 = &s[..5];  // 最初から4まで
    let world2 = &s[6..];  // 6から最後まで
    let s2 = &s[..];       // 最初から最後まで
    println!("{}, {}, {}, {}, {}, {}", s, hello, world, hello2, world2, s2);

    // &strは文字列スライス
    // &strで、std:string:Stringの参照も受け取れる
    fn first_word(s: &str) -> &str {
        // 文字列の各値を確認するためバイト列に変換
        let bytes = s.as_bytes();

        // iterはコレクションの各要素を変える
        // enumerateはiterをラップしてタプル(添字,参照)を返す
        for (i, &item) in bytes.iter().enumerate() {
            // バイトリテラルを使って空白かどうかをチェック
            if item == b' ' {
                // 最初から空白の1つ手前までの文字列スライスを返す
                return &s[0..i];
            }
        }
        &s[..]
    }
    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear();
    // ここで文字列を空にはできない
    // sの不変な参照wordが発生しているため

    println!("first word is {}", word);
    s.clear();

    // 文字リテラルは文字列スライス
    let word = first_word("hogehoge fugafuga piyopiyo");
    println!("first word is {}", word);

    // 配列のスライスもできる
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice first part is {}", slice[0]);
}