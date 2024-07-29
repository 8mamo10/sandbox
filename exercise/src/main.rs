fn main() {
    proconio::input! {
        n: usize,
        mut l: [usize; n],
    }
    l.sort();
    let mut ans = 0;
    for i in 0..l.len() {
        for j in 0..i {
            for k in 0..j {
                if l[i] != l[j] && l[j] != l[k] && l[i] < l[j] + l[k] {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
