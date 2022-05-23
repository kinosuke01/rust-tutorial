// 結合テストは、testsディレクトリ配下に書く
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::it_adds_two(2));
}
