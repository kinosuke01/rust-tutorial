use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

// 各構造体や関数はpublicにしないと、main.rsから呼べない
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        // cloneを使用する代償
        // https://doc.rust-jp.rs/book-ja/ch12-03-improving-error-handling-and-modularity.html#clone%E3%82%92%E4%BD%BF%E7%94%A8%E3%81%99%E3%82%8B%E4%BB%A3%E5%84%9F
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // use std::error::Error で Error が使用可能になる
    // Box<dyn Error>は、関数がErrorトレイトを実装する型を返すことを意味する

    // ファイルオープンにエラーしたときにErrを返す
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();

    // ファイル読み込みに失敗したときにErrを返す
    f.read_to_string(&mut contents)?;

    // これでもOK
    // use std::fs;
    // let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
}
