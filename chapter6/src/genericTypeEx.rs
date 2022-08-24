enum Option<T> {
    None,
    Some(T),
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Option::None => Option::None,
        Option::Some(i) => Option::Some(i + 1),
    }
}
fn main() {
    let five = Option::Some(5);
    let six = plus_one(five);
    let none = plus_one(Option::None);
}
