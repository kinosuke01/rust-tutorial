// 構造体にもpubをつける
pub struct Breakfast {
    // 構造体の公開要素にはpubをつける
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peachese"),
        }
    }
}

// enumに限り、
// pubをつけるとすべての要素が公開される
pub enum Appetizer {
    Soup,
    Salad,
}
