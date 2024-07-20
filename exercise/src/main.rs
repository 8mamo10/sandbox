fn main() {
    proconio::input! {
        n: usize,
        v: [(i64, i64); n],
    }
    let mut sum = 0;
    for (a, b) in &v {
        sum += (a + b) * (b - a + 1) / 2;
    }
    println!("{}", sum);
}
