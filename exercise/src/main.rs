fn main() {
    proconio::input! {
        s: String,
    }
    match s.as_str() {
        "ABC" => println!("ARC"),
        "ARC" => println!("ABC"),
        _ => unreachable!(),
    }
}
