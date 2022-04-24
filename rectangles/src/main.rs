#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体にメソッドを追加
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The areea of reactangle is {} square pixels.",
        rect1.area()
    );

    // これはエラーする
    // println!("rect1 is {}", rect1);

    // struct Rectangleの上に#[derive(Debug)] を追加することで、
    // {:?} でデバッグ用出力が可能になる。{:#?} にすると改行されて出力される
    // Rustには、derive注釈で使えるトレイト(インターフェース?)が多く提供されている
    println!("rect1 is {:?}", rect1);

}

