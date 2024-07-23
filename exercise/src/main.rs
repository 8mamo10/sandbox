fn main() {
    proconio::input! {
        n: usize,
        d: usize,
        v: [(f64, f64); n],
    }
    let mut ans = 0;
    for &(x, y) in &v {
        if x.hypot(y) <= d as f64 {
            ans += 1;
        }
    }
    println!("{}", ans);
}
