fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut p: [usize; n],
    }
    p.sort();
    let mut ans = 0;
    for i in &p[0..k] {
        ans += i;
    }
    println!("{}", ans);
}
