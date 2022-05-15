use std::io;            // 標準ライブラリのioモジュールを読み込み
use std::cmp::Ordering; // 列挙体 Less, Greater, Equal から成る
use rand::Rng;          // Rngトレイトの呼び出し (トレイト=Rubyのmoduleみたいなもの?)

fn main() {
    println!("Guess the number!");

    // thread_rng: 乱数生成器を返す
    // gen_range: 乱数を返す(Rngで定義された関数?), 上限値は戻り値に含まないため1..101としている
    let secret_number = rand::thread_rng().gen_range(1..101);
    // println!("The secret number is: {}", secret_number);

    loop { // 無限ループ
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

        // guessは定義済だったが、型替え再定義して上書きできる(シャドーイング)
        // trimで入力値に含まれる改行コードを除去
        // parseで文字列を解析して数値に変換する
        // u32は非負整数の型。数値はデフォルトはi32で、32ビットの整数を表す
        let guess: u32 = match guess.trim().parse() {
            // Resultをmatchで受け取り、内容に応じてハンドリング
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess < 1 || guess > 100 {
            println!("The secret number will be between 1 and 100");
            continue;
        }

        // {}はプレースホルダーにあたる
        println!("You guessed: {}", guess);

        // `cmp`メソッド
        //   2値を比較する
        //   引数には比較対象の参照を指定する
        //   戻り値は、Ordering
        // `match`式
        //   cmpの結果の値を受け取り、{}内のkeyと照合する。
        //   照合すれば、対応するvalを実行する
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break; // 無限ループ終了
            },
        }
    }
}