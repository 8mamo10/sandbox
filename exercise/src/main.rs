fn main() {
    proconio::input! {
        s: String,
    }
    let mut ans = 0;
    let mut count = 0;
    for c in s.chars() {
        if c == 'R' {
            count += 1;
        } else {
            ans = ans.max(count);
            count = 0;
        }
    }
    println!("{}", ans.max(count));
}
