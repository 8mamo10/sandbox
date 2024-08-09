fn main() {
    proconio::input! {
        s: proconio::marker::Chars,
    }

    for i in 0..s.len() {
        if i % 2 == 0 {
            if s[i].is_ascii_uppercase() {
                println!("No");
                return;
            }
        } else {
            if s[i].is_ascii_lowercase() {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
