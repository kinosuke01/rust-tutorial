// 標準ライブラリのioモジュールを読み込み
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");

    // imuutable(不変)な変数の定義
    // 変数はデフォルトで不変
    // let foo = bar;

    // muutable(可変)な変数の定義
    // mutと指定することで可変となる
    // String::newはString型の関連関数(=スタティックメソッド)
    // ここでは空の文字列を生成している
    let mut guess = String::new();

    // std::io::Stdinオブジェクトを返す
    io::stdin()
        // read_lineで入力受付
        // 引数として与えた変数の値を変える
        // io::Result(Ok or Errを持つenum)を返す
        // &で参照渡し, mutで可変
        .read_line(&mut guess)  
        // Errだった場合にメッセージを残してクラッシュ
        .expect("Failed to read line");

    // {}はプレースホルダーにあたる
    println!("You guessed: {}", guess);
}
