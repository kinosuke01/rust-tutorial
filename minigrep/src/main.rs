use std::env;

fn main() {
    // コマンドライン引数を受け取る
    // args()で受け取るイテレータを、collect()でベクタに変換
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);
}
