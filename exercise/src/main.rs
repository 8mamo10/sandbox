fn main() {
    proconio::input! {
        n: String,
    }
    let mut sum = 0;
    for c in n.chars() {
        sum += c.to_digit(10).unwrap();
    }
    match sum % 9 {
        0 => println!("Yes"),
        1..=8 => println!("No"),
        _ => unreachable!(),
    }
}
