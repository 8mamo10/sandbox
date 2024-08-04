fn main() {
    proconio::input! {
        _n: usize,
        x: usize,
        s: proconio::marker::Chars,
    }
    let mut ans = x;
    for c in s {
        if c == 'o' {
            ans += 1;
        } else {
            if ans == 0 {
                continue;
            }
            ans -= 1;
        }
    }
    println!("{}", ans);
}
