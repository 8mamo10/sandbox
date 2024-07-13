fn main() {
    proconio::input! {
        n: i32,
        x: i32,
        t: i32,
    }
    let cnt = n / x;
    let ans;
    if n % x != 0 {
        ans = cnt + 1;
    } else {
        ans = cnt;
    }
    println!("{}", ans * t);
}
