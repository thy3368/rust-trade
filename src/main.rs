use std::fmt::{Display, Formatter};

struct Price {
    order_no: String
}

impl Display for Price {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");

    let order = Price { order_no: String::from("123456") };
    println!("{}", order);

}
