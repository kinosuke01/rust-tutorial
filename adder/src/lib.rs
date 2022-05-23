#![allow(unused)]
struct DummyStruct {
    name: String
}
impl DummyStruct {
    fn is_hogehoge(&self) -> bool {
        true
    }

    fn do_panic(&self) {
        panic!("PanicPanic!!");
    }
}
pub fn it_adds_two(i: i32) -> i32{
    i + 2
}

// 単体テスト(モジュールのテスト)では、
// 各ファイルにtestsという名前のモジュールを作り、テスト関数を実装、
// そのモジュールをcfg(test)で注釈することが、慣習となっている
// 
// cfg(test)属性を追加することで、cargo testのときだけコンパイル対象となる
#[cfg(test)]
mod tests {
    // 外部モジュールで定義されたテスト対象すべてを、
    // testsモジュール内部に取り込む
    use super::*;

    // #[test] と注釈することでtest用関数と判断される
    // テスト用関数はメインスレッドの子スレッドとして実行される
    // 子スレッドがpanicしたとき、失敗としてカウントされる
    #[test]
    fn it_works() {
        let result = 2 + 2;
        // 値が同じか
        // 値が異なるとpanic!する
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works2() {
        // 値が異なるか
        assert_ne!(5, 4);
    }

    #[test]
    fn another() {
        // panicさせたらテストは失敗する
        // panic!("Make this test fail");
    }

    #[test]
    fn test_is_hogehoge() {
        let dummy = DummyStruct{name: String::from("taro")};
        // 値がtrueか
        // 値がfalseだと失敗する
        assert!(dummy.is_hogehoge());
    }

    #[test]
    fn test_comment() {
        println!("HOGHEOGHEGOEHGOEHGOEHGOEHGO {}", 111);
        // 第2引数以降にformat!と同じ渡し方でコメントを差し込める
        assert!(true, "This value should be {}", true);
    }

    // panic発生をチェックするときは、#[should_panic]属性を追加する
    // expectedで、失敗メッセージに含まれるべき文字列を指定できる
    #[test]
    #[should_panic(expected = "Panic!")]
    fn test_panic() {
        let dummy = DummyStruct{name: String::from("jiro")};
        dummy.do_panic();
    }
}
