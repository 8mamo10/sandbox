fn main() {
    proconio::input! {
        n: usize,
        x: [f64; n],
    }

    let mut man: f64 = 0.;
    let mut euc: f64 = 0.;
    let mut che: f64 = 0.;
    for &i in &x {
        man += i.abs();
        euc += i * i;
        che = i.abs().max(che);
    }
    println!("{}", man);
    println!("{}", euc.sqrt());
    println!("{}", che);
}
