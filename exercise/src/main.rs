fn main() {
    proconio::input! {
        a: char,
    }
    match a {
        'a'..='z' => println!("a"),
        'A'..='Z' => println!("A"),
        _ => unreachable!(),
    }
}
