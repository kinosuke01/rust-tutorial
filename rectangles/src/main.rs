#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 構造体にメソッドや関連関数を追加
impl Rectangle {
    // 面積を返すメソッド
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // selfに完全にはめ込まれているか？
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 正方形のRectangleを返す関連関数
    // メソッドではないので引数にselfを取らない
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};

    // 参照からも実体からもメソッド呼び出しできる
    // rect1.area();
    // (&rect1).area();

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

    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(50);
    println!("square is {:?}", square);
}

