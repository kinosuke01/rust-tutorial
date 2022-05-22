#![allow(unused)]
struct DummyStruct {
    name: String
}
impl DummyStruct {
    fn is_hogehoge(&self) -> bool {
        true
    }
}

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
}
