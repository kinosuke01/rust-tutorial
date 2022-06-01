use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fmt::Display;
use std::thread;
use std::time::Duration;

fn main() {
    println!("Hello, world!");

    tup_fn();
    array_fn(3);
    array_fn(14);
    println!("{}", plus_one(10));
    if_fn();
    while_fn();
    for_fn();
    ownership_fn();
    ref_fn();
    slice_fn();
    struct_fn();
    enum_fn();
    vec_fn();
    string_fn();
    hashmap_fn();
    panic_fn();
    error_fn();
    generics_fn();
    lifetime_fn();
    closure_fn();
    iterator_fn();
    box_fn();
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

fn struct_fn() {
    // 構造体の定義
    struct User {
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    // 構造体のインスタンス作成
    // mutで構造体全体を可変にできる(一部フォールドだけ可変にはできない)
    let mut user1 = User {
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotherremail@example.com");
    println!("user1 struct: {}, {}, {}, {}", user1.username, user1.email, user1.active, user1.sign_in_count);

    fn build_user(email: String, username: String) -> User {
        User {
            email,    // 引数emailが自動で代入
            username, // 引数usernameが自動で代入
            active: true,
            sign_in_count: 1,
        }
    }
    let user2 = build_user(String::from("hogeo@example.com"), String::from("hogeo"));
    println!("user2 struct: {}, {}, {}, {}", user2.username, user2.email, user2.active, user2.sign_in_count);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("another"),
        ..user1  // 他のインスタンスの値を引き継ぐことができる
    };
    println!("user3 struct: {}, {}, {}, {}", user3.username, user3.email, user3.active, user3.sign_in_count);

    // タプル構造体
    struct Color(i32, i32, i32);
    let black = Color(0, 0, 0);
    println!("r = {}, g = {}, b = {}", black.0, black.1, black.2);
}

fn enum_fn() {
    // enumの定義
    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6,
    }
    let four = IpAddrKind::V4; // IpAddrKind型でV4を表す
    let six = IpAddrKind::V6;  // IpAddrKind型でV6を表す
    println!("{:?}, {:?}", four, six);

    // 列挙子とデータは紐付けできる
    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?}, {:?}", home, loopback);

    // IpAddrに関する標準ライブラリある
    // https://doc.rust-lang.org/std/net/enum.IpAddr.html

    enum Message {
        // Quit,
        // Move {x: i32, y: i32},
        Write(String),
        // ChangeColor(i32, i32, i32),
    }
    // enumにメソッドを追加できる
    impl Message {
        fn call(&self) {
            // ここに処理を書く
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();

    /*
        存在するかしないかを表す列挙体Option<T>
        標準ライブラリで定義されている
        https://doc.rust-lang.org/std/option/enum.Option.html

        enum Option<T> {
            Some(T),
            None,
        }
        Option::なし、Some,Noneだけで使える
    */

    // let some_number = Some(5);
    // let some_string = Some("a string");
    // Noneを設定する場合は、
    // Someの型がわからないので指定する必要あり
    // let absent_number: Option<i32> = None;

    // matchを使って、列挙子ごとの挙動を切り替える
    #[derive(Debug)]
    enum UsState {
        Alaska,
    }
    enum Coin {
        Peny,
        Nickel,
        Dime,
        Quarter(UsState),
    }
    fn value_in_cents(coin: Coin) -> u32 {
        match coin {
            Coin::Peny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }
    value_in_cents(Coin::Peny);
    value_in_cents(Coin::Nickel);
    value_in_cents(Coin::Dime);
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // Optionで処理を切り分ける
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1), // Some(5) はマッチする
        }
    }
    let five = Some(5);
    plus_one(five);
    plus_one(None);

    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (), // 他のすべてにマッチする
    }
    // 上記と同じように振る舞う if let
    if let Some(3) = some_u8_value {
        println!("three");
    }
 
    // if let では elseも使える
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
        println!("count value is {}", count);
    }
}

fn vec_fn() {
    // ベクタはサイズを変更可能な配列
    // https://doc.rust-jp.rs/rust-by-example-ja/std/vec.html
    // https://doc.rust-lang.org/std/vec/

    // ベクタの作成と更新
    // 他の変数と同じくスコープを抜けたら、メモリから解放される
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // vec!は、ベクタを生成するマクロ
    let v = vec![1, 2, 3, 4, 5];

    // 添字で要素の参照にアクセス
    // 存在しない要素にアクセスするとpanicになる
    // ※要素の不変参照が発生したため、vは可変にはできない
    let third: &i32 = &v[2];
    println!("The third elementis {}", third);

    // getメソッドで要素にアクセスする(Option<&T>で値を得られる)
    match v.get(2) {
        Some(third) => println!("The third elementis {}", third),
        None => println!("The is no third element."),
    }

    // forを使って、ベクタの各要素にアクセス
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    // 要素の更新を行う
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        // *iで参照外しをして値を書き換える
        *i += 50;
    }
    // vec is [150, 82, 107]
    println!("vec is {:?}", v);

    // ベクタ型の各要素に違う型を使いたい場合は、enumを活用する
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row is {:?}", row);
}

// ref: https://doc.rust-jp.rs/book-ja/ch08-02-strings.html
fn string_fn() {
    // 文字列の生成
    // let mut s = String::new();

    // 文字列リテラルを文字列に変換(1)
    let data = "initial contents";
    let s = data.to_string();
    println!("{}", s);

    // 文字列リテラルを文字列に変換(2)
    let s = "initial contents".to_string();
    println!("{}", s);

    // 文字列リテラルを文字列に変換(3)
    let s = String::from("initial contenst");
    println!("{}", s);

    // 文字列の更新
    // push_strメソッドでStringに文字列スライスを追記する
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    // s2は参照渡しなので、所有権はpush_strに奪われない
    println!("s2 is {}", s2);

    // 文字列の連結
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; 
    // + 演算子の実体はこんな感じ
    // fn add(self, s: &str) -> String {
    // - s1はムーブされもう使用できない
    // - s2は&strじゃないけど"参照外し型強制"が実行されるので使えるらしい。
    //   s2は参照渡しなので引き続き使える
    println!("{}", s3);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    // format!マクロだとムーブされることなく、文字列の連結ができる
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("{}", s);

    // Rustは文字列に対して添字アクセスできない
    // 文字列はマルチバイトを含み、文字列はVec<u8>のラッパで1要素は1バイトから成ることから
    // 添字で値を取得しても1文字が取得できるとは限らないため
    // let s1 = String::from("hello");
    // let h = s1[0]; // error!!

    let hello = String::from("こんにちは");
    // lenは文字の長さではなくバイト数を取得する
    let len = hello.len();
    println!("len is {}", len);
    // マルチバイト文字を、1文字分割するところでスライスするとpanic発生
    // let s = &hello[0..4];
    // println!("s is {}", s);

    // マルチバイト文字を1文字ずつ取り出したいときはchar()を使う
    for c in hello.chars() {
        println!("{}", c);
    }
    // バイトごとに出力する
    for b in hello.bytes() {
        println!("{}", b);
    }
}

fn hashmap_fn() {
    // ハッシュマップを使う場合はuseが必要
    // 生成するための組み込みマクロはない
    use std::collections::HashMap;

    // {"Blue" => 10, "Yellow" => 5} を作成
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 5);
    println!("{:?}", scores);

    // tarms.iter().zip(initial_scores.iter()) で
    // ("Blue", 10), ("Yellow", 50) のようなタプルを要素とするイテレータができる
    // https://doc.rust-lang.org/core/iter/trait.Iterator.html#method.zip
    // これにcollectメソッドをかませると、HashMapになる
    let teams = vec![String::from("Blue"), String::from("Yelllow")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _>  = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    // field_name,field_valueの所有権はハッシュにムーブする
    map.insert(field_name, field_value);

    // 要素へのアクセス
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    // getメソッドで参照できる
    let score = scores.get(&team_name);
    match score {
        Some(x) => {
            println!("{} socore is {}", team_name, x);
        },
        _ => {},
    }
    // forループで走査できる
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // 要素の更新
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    // 同じキーを指定すると上書きされる
    scores.insert(String::from("Blue"), 25);
    if let Some(x) = scores.get("Blue") {
        println!("Blue score is {}", x);
    }
    // 値がないときだけ更新するケース
    // entryはenumで、値の有無の確認ができる
    // or_insertは、キーが存在するときは値の可変参照を返す
    // なければ、引数を値として挿入して、可変参照を変えす
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(100);
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wondeful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        // 値の可変参照を返す(値がなければ0を挿入して返す)
        let count = map.entry(word).or_insert(0);
        // 可変参照の参照外しをしてインクリメント
        // 参照なのでhashの値が更新される
        *count += 1;
    }
    println!("{:?}", map);
}

fn panic_fn() {
    // RUST_BACKTRACE=1 cargo run
    // のように環境変数をセットするとpanic発生時にBACKTRACEが得られる
    // panic!("crash and burn");
}

fn error_fn() {
    let file_path = "./hello.txt";
    let f = File::open(file_path);
    let f = match f {
        Ok(file) => file,
        // ファイルが無い場合
        // マッチガード(matchのオマケ機能)で更に細かい条件分岐を行っている
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            // ファイルを新規作成する
            match File::create(file_path) {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        "Tried to create file but there was a problem: {:?}",
                        e
                    );
                }
            }
        },
        // その他のエラーの場合
        Err(error) =>  {
            panic!(
                "There was problem opening the file {:?}",
                error
            )
        }
    };
    println!("file info is {:?}", f);

    // Errのときはすぐにpanicさせる場合
    // let f = File::open("notfound.txt").unwrap();
    // エラーメッセージを指定してpanicさせるとき
    // let f = File::open("notfound.txt").expect("Failed to open file");

    // エラーの移譲
    // matchでエラーを呼び出しもとのコードに返す関数
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e)
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    match read_username_from_file() {
        Ok(s) => println!("username is {}", s),
        Err(e) => println!("error is {:?}", e),
    }
    // read_username_from_fileの簡略形式
    fn read_username_from_file2() -> Result<String, io::Error> {
        // 末尾を ? とすることで、
        // Errの場合に関数の戻り値として指定された型に変換してreturnする
        let mut f = File::open("hello.txt")?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)

        // さらに簡略化すると
        // File::open("hello.txt")?.read_to_string(&mut s)?;
        // Ok(s)
    }
    match read_username_from_file2() {
        Ok(s) => println!("username is {}", s),
        Err(e) => println!("error is {:?}", e),
    }

    // 値エラーを構造体に閉じ込める
    pub struct Guess {
        value: u32,
    }
    impl Guess {
        pub fn new (value: u32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}", value);
            }

            Guess {
                value
            }
        }

        pub fn value(&self) -> u32 {
            self.value
        }
    }
    let guess = Guess::new(12);
    println!("Guess.value is {}", guess.value());
}

// https://doc.rust-jp.rs/book-ja/ch10-02-traits.html
fn generics_fn() {
    // ジェネリクスを使用しても、
    // 単相化(monomorphization)が行われるため、
    // コードのパフォーマンスに大きく影響はしない
    // https://doc.rust-jp.rs/book-ja/ch10-01-syntax.html

    // ベクタのスライスを取り、最大値の要素を返す
    fn largest_i32(list: &[i32]) -> i32 {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let number_list = vec![23, 50, 25, 100, 65];
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    // 文字のスライスを取り、最大値の要素を返す
    fn largest_char(list: &[char]) -> char {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest_char(&char_list);
    println!("The largest char is {}", result);

    // ジェネリックな型を持つ構造体
    struct Point<T> {
        x: T,
        y: T,
    }
    // メソッド追加
    // implのあとに<T>を指定することで、
    // コンパイラがPoint<T>のTをジェネリックな型であると判断できる
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }
    let i_struct = Point {x: 5, y: 10};
    println!("Point x is {}", i_struct.x());    

    // 2つの型をもたせるケース
    struct PointType2<T, U> {
        x: T,
        y: U,
    }
    impl<T, U> PointType2<T, U> {
        // <V>をメソッド名のあとに指定することで、
        // Vがジェネリックな型であるとコンパイラに判断させる
        fn mixup<V>(self, other: Point<V>) -> PointType2<T, V> {
            PointType2 {
                x: self.x,
                y: other.y,
            }
        }
    }
    let if_struct = PointType2 {x: "hogehoge", y: 5.1};
    let mixup_struct = if_struct.mixup(i_struct);
    println!("mixup_struct x: {}, y: {}", mixup_struct.x, mixup_struct.y);

    // トレイト(=インターフェースのようなもの)
    pub trait Summary {
        fn summarize(&self) -> String;
        fn read_more_author(&self) -> String;

        // デフォルト実装
        // トレイトを組み込んだ先で実装しなくても使える
        fn read_more(&self) -> String {
            format!("(Read more from {}...)", self.read_more_author())
        }
    }
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    // Summaryトレイトを構造体上に実装する
    impl Summary for NewsArticle {
        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }

        fn read_more_author(&self) -> String {
            format!("@{}", self.author)
        }
    }
    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
    };
    println!("news summary: {}, {}", news_article.summarize(), news_article.read_more());

    // 引数にトレイトを指定する場合は、&implキーワードと共にトレイト名を渡す
    pub fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }
    // impl Trait構文は、以下"トレイト境界構文"の糖衣構文にあたる
    // pub fn notify<T: Summary>(item: &T) {}
    notify(&news_article);

    // 複数のトレイト境界を指定する場合
    // pub fn notify(item: &(impl Summary + Display)) {}
    // pub fn notify<T: Summary + Display>(Item: &T) {}

    // トレイトを返す関数の書き方
    fn returns_summarizable() -> impl Summary {
        NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        }
    }
    let something = returns_summarizable();
    println!("something summary is {}", something.summarize());

    // なんらかの型Tに関してジェネリックである
    // PartialOrdトレイトで比較(<や>)ができる
    // Copyトレイトでスライスからの要素取り出しでcopyができる
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }
        largest
    }
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // ブランケット実装
    // ジェネリックな型が特定の条件を満たしている場合にだけ実装されるメソッド
    struct Pair<T> {
        x: T,
        y: T,
    }
    impl<T> Pair<T> {
        fn new(x: T, y: T) -> Self {
            Self {x, y}
        }
    }
    impl<T: Display + PartialOrd> Pair<T> {
        fn cmp_display(&self) {
            if self.x >= self.y {
                println!("The largest member is x = {}", self.x);
            } else {
                println!("The largest mumber is y = {}", self.y);
            }
        }
    }
    let pair = Pair::new(10, 100);
    pair.cmp_display();
}

fn lifetime_fn() {
    // この実装ではコンパイルが通らない
    // 戻り値の&strがstr1とstr2のどちらの参照なのかわからないので
    // この関数を呼び出した処理が、戻り値の破棄のタイミングがわからない
    // fn longest(str1: &str, str2: &str) -> &str {
    //     if str1.len() > str2.len() {
    //         str1
    //     } else {
    //         str2
    //     }
    // }

    // そこで、渡す引数のライフタイムは同一であると注釈をつける
    // 以下はライフタイム'aを定義している
    // これにより渡した引数の破棄タイミング(ライフタイム)が短い方が適用されて
    // 戻り値として返される
    fn longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
        if str1.len() > str2.len() {
            str1
        } else {
            str2
        }
    }

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz";
        result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    } // 短いライフタイムで揃えられるので、resultはここでスコープを抜ける

    // 構造体の要素に参照を保持する場合、ライフタイム注釈を加える必要がある
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("Call me Ishmeal. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a 'a'");
    // iは、first_sentenceより長生きできない。共に破棄される。
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("ImportantExcerpt is {}", i.part);

    // ライフタイム省略
    // fn first_word(s: &str) -> &str {
    // は、以下のように解釈されてコンパイルされる
    // fn first_word<'a>(s: &'a str) -> &'a str {

    impl<'a> ImportantExcerpt<'a> {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            // メソッドが参照を返す場合、ライフタイムはselfに沿う
            self.part
        }
    }
    i.announce_and_return_part("next station number is 12");

    // staticなライフタイム指定
    // バイナリに含まれる値になり、破棄されない
    let s: &'static str = "I have a static lifetime.";
    println!("static lifetime variable is [{}]", s);

    // ジェネリクスとトレイトライフタイムのあわせ技
    fn longest_with_an_announcement<'a, T>(
        x: &'a str,
        y: &'a str,
        ann: T,
    ) -> &'a str
    where
        T: Display,
    {
        println!("Announcement! {}", ann);
        if x.len() > y. len() {
            x
        } else {
            y
        }
    }
    println!("{}", longest_with_an_announcement("10", "20", "hello everyone!!"));
}

fn closure_fn() {
    // クロージャの定義
    // 匿名関数を変数に代入する
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(0));
        num
    };
    expensive_closure(5);

    // 型注釈をもたせることもできるが、
    // 基本的にコンパイラが型推論をしてくれるので書かなくてもよい
    // let expensive_closure = |num: u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    // クロージャはFn, FnMut, FnOnce のいずれかのトレイトを持っている
    // calculationには、トレイと境界を指定したジェネリックな型を設定
    struct Cacher<T>
        where T: Fn(u32) -> u32
    {
        calculation: T,
        value: Option<u32>,
    }
    impl<T> Cacher<T>
        where T: Fn(u32) -> u32
    {
        // 引数としてクロージャを受け取る
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        // 遅延評価メソッドを定義
        // 値があればそれを返し、なければ取得とvalueセットして値を返す
        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }
    // クロージャを渡して構造体を作成
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(0));
        num
    });
    expensive_result.value(2);

    // クロージャは自身が定義されたスコープにある変数にアクセスできる
    let x = 4;
    let equal_to_x = |z| z == x;
    let y = 4;
    assert!(equal_to_x(y));

    // moveを指定するとクロージャに所有権がムーブする
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // println!("cant use x here: {:?}", x);
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

    // クロージャのトレイト
    // FnOnce: 外の変数をムーブする
    // FnMut: 可変借用する
    // Fn: 不変借用する
}

fn iterator_fn() {
    // イテレータの設定
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // forはv1_iterの所有権をムーブする
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // イテレータのトレイトはこんな感じ
    /*
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    */

    // nextで要素を順に取り出すことができる
    // イテレータ内部でシーケンスが変わっているのでmut指定が必要
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

    // sumはv1_iterの所有権を奪う
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    // イテレートアダプタ - 他の種類にイテレータに変換する
    let v1: Vec<i32> = vec![1, 2, 3];
    // mapだけでは何も起こらないので
    // v1.iter().map(|x| x + 1);
    // collectでコレクション型に変換する必要がある
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        // into_iter は所有権を奪うイテレータを作成する
        // (可変参照を繰り返したい場合は iter_mut を使用する)
        // filterでshoe_sizeに該当するものだけを抽出する
        shoes.into_iter()
            .filter(|s| s.size == shoe_size)
            .collect()
    }
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    assert_eq!(
        in_my_size,
        vec![
            Shoe { size: 10, style: String::from("sneaker") },
            Shoe { size: 10, style: String::from("boot") },
        ]
    );

    // Iteratorトレイトで独自のイテレータを作成する
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    // Iteratorトレイトを指定してnextメソッドを実装すればよい
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            self.count += 1;
            if self.count < 6 {
                Some(self.count)
            } else {
                None
            }
        }
    }
    let counter = Counter::new();
    for c in counter {
        println!("Got: {}", c);
    }

    // zipで (1, 2), (2, 3), (3, 4), (4, 5) になる
    // mapで 2, 6, 12, 20
    // filterで 6, 12
    // sumで 18
    let sum: u32 = Counter::new().zip(Counter::new().skip(1))
                    .map(|(a, b)| a * b)
                    .filter(|x| x % 3 == 0)
                    .sum();
    assert_eq!(18, sum);

    // イテレータは、Rustのゼロコスト抽象化の１つ
    // https://doc.rust-jp.rs/book-ja/ch13-04-performance.html
}

fn box_fn() {
    // ListがListを再帰的に呼び出す構造
    // ListがListの実体を呼び出すとメモリ確保容量が計算できないため、
    // コンパイルできない
    // Boxで囲うことでポインタになり容量計算ができるようになる
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    use List::{Cons, Nil};

    // 再帰的に呼び出す
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));

    match list {
        Cons(i, b) => {
            assert_eq!(1, i);
            println!("{:?}", b);
        },
        Nil => {},
    };

    // BoxはDerefトレイトを実装しているため、参照のように扱える
    // Box<T>がスコープを抜けると実体もメモリから削除される(Dropトレイトの実装)
}
