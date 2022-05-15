use std::fs::File;
use std::io;
use std::io::Read;
use std::io::ErrorKind;

fn main() {
    println!("Hello, world!");

    tup_fn();
    array_fn(3);
    array_fn(14);
    println!("{}", plus_one(10));
    if_fn();
    while_fn();
    for_fn();

    // 所有権について
    ownership_fn();

    // 参照渡し
    ref_fn();

    // スライス
    slice_fn();

    // 構造体
    struct_fn();

    // Enum
    enum_fn();

    // ベクタ
    vec_fn();

    // 文字列
    string_fn();

    // ハッシュマップ
    hashmap_fn();

    panic_fn();

    error_fn();
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
}
