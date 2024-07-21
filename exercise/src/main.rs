fn main() {
    proconio::input! {
        n: usize,
        a: [i64; n],
    }
    let mut max = 0;
    let mut ans = 0;
    for i in &a {
        max = max.max(*i);
        ans += max - i;
    }
    println!("{}", ans);
}
