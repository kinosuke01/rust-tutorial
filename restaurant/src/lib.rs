// # 用語
// - パッケージ: クレートをビルドし、テストし、共有できるCargoの機能
// - クレート: ライブラリか実行可能ファイルを生成する、木構造をしたモジュール群
// - モジュールとuse: パスの構成、スコープ、公開の有無を決定できる
// - パス: 要素に名前をつける方法

// cargoは以下の扱いをする
// main.rsをバイナリクレートのルート
// lib.rsをライブラリクレートのルート

// モジュールの定義
// 実装は front_of_house.rs, back_of_house
mod front_of_house;
mod back_of_house;

// useを使うことで、直接hostingを呼び出すことができる
// 慣習的に関数を呼び出すときは、関数を直接useするのではなく
// 上位のモジュールをuseする
// 関数がどこで定義されたものかわかりずらくなるため
use self::front_of_house::hosting;

// pub use にすると、外部からもこのモジュールが使えるようになる
// pub use self::front_of_house::hosting;

pub fn eat_at_restrant() {
    // 接待パスの呼び出し
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パスでの呼び出し
    front_of_house::hosting::add_to_waitlist();

    // hostingをuseしているのでこれでもOK
    hosting::add_to_waitlist();

    // 夏にライ麦パン付き朝食を注文
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // やっぱり別のパンにする
    // 公開要素なので外部から上書き可能
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // 季節のフルーツは知ることも修正することもできない
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
