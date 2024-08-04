fn main() {
    proconio::input! {
        _: i32,
        mut x: isize,
        s: String,
    }
    for c in s.chars() {
        match c {
            'o' => x += 1,
            'x' => x = (x - 1).max(0),
            _ => unreachable!(),
        }
    }
    println!("{}", x);
}
