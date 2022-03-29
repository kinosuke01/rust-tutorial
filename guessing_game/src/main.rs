use std::io;   // 標準ライブラリのioモジュールを読み込み
use rand::Rng; // Rngトレイトの呼び出し (トレイト=Rubyのmoduleみたいなもの?)

fn main() {
    println!("Guess the number!");

    // thread_rng: 乱数生成器を返す
    // gen_range: 乱数を返す(Rngで定義された関数?), 上限値は戻り値に含まないため1..101としている
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Thesecret number is: {}", secret_number);

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
