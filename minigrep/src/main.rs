use std::env;
use std::process;

use minigrep::Config;

fn main() {
    // コマンドライン引数を受け取る
    // args()で受け取るイテレータを、collect()でベクタに変換
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);

        // std::process 読み込みで使用可能
        process::exit(1);
    });

    // for debug
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
