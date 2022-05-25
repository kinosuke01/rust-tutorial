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

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    // for debug
    // println!("With text:\n{}", contents);

    Ok(())
}

// 戻り値ベクタの要素は、contentsの一部を切り取ったものなので、
// ライフタイムはcontentsのものに揃える
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // to_lowercase で std::string::String が生成される
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // containsには文字列の参照を渡す
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}