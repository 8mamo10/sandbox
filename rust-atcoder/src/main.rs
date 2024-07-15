use proconio::input;

fn main() {
    println!("Hello, world!");
    /* Chapter 03: Hello, world! */
    // New line
    println!();
    // Standard error
    eprintln!("Hello, world3!");

    /* Chapter 04: Arithmetic operation */
    // Place holder
    println!("{} {}", 2, "times");
    // Arithmetic operation
    println!("2 + 3 = {}", 2 + 3);
    println!("2 * 3 = {}", 2 * 3);
    println!("{}", 6.5 / 2.5);

    /* Cahpter 05: Variables and types */
    let length;
    length = 5;
    println!("Area={}", length * length);

    /* Cahpter 06: Literal */
    // String literal
    println!("\"Fool,\" said I, \"you do not know\"");
    println!(
        "/\\
\\/"
    );
    // Raw string literal
    println!(r"\\\\\\\\\\");
    println!(r#""Fool," said I, "you do not know""#);
    // Integer literal
    //let big_number: i32;
    let big_number: i64;
    big_number = 2147483648;
    println!("{}", big_number);
    let big_number2;
    big_number2 = 2147483648_i64;
    println!("{}", big_number2);
    // Floating-point literal
    let avogadro_constant;
    avogadro_constant = 6.02e+23;
    println!("{}", avogadro_constant);

    /* Chapter 07: Receipt of inputs */
    // Input
    //proconio::input! {
    input! {
        n: i32,
        a: i32,
        b: i32,
    }
    println!("{}", n - a + b);

    /* Chapter 08: if expression */
    let x = 10;
    if x < 10 {
        println!("small");
    } else if x > 10 {
        println!("large");
    } else {
        println!("10");
    }

    /* Chapter 09: Block and scope */
    // block
    println!("before");
    {
        println!("in");
    }
    println!("after");
    // shadowing
    let sh = 10;
    println!("{}", sh);
    let sh = 20;
    println!("{}", sh);
    {
        println!("{}", sh);
        let sh = 30;
        println!("{}", sh);
    }
    println!("{}", sh);
    // block returns value
    println!("before block");
    let hoge = {
        println!("in block");
        10
    };
    println!("after block: {}", hoge);
    let x = -1;
    let abs;
    abs = if x >= 0 { x } else { -x };
    println!("abs = {}", abs);

    /* Chapter 10: Assert */
    // assert!
    input! {
        x: i32,
    }
    //let r = x % 10;
    let r = x.rem_euclid(10);
    assert!(0 <= r && r < 10);
    println!("remaining is {}", r);
    // assert_eq! / assert_ne!
    input! {
        x: i32,
        y: i32,
    }
    let rounded = x / y * y;
    println!("rounded us {}", rounded);
    assert_eq!(rounded % y, 0);
    // panic
    input! {
        x: i32,
    }
    if x != 0 {
        let y = 100 / x;
        println!("{}", y);
    } else {
        println!("cannot divide by 0");
    }

    /* Chapter 11: Tuple  */
    // tuple
    let tuple: (i32, f64, i32) = (10, 2.5, 20);
    println!("1st: {}", tuple.0);
    println!("2nd: {}", tuple.1);
    println!("3rd: {}", tuple.2);
    // pattern match
    let tuple = (10, 2.5);
    let (x, y) = tuple;
    assert_eq!(x, 10);
    assert_eq!(y, 2.5);
    // block returns tuple
    input! {
        a: i32,
        b: i32,
    }
    let (max, min) = if a > b { (a, b) } else { (b, a) };
    assert!(max >= min);
    println!("big: {}", max);
    println!("small: {}", min);
    // unit
    let unit;
    unit = {
        println!("returs ()");
    };
    assert_eq!(unit, ());

    /* Chapter 12: Array */
    let x: isize = 5;
    println!("x: {}", x);
    println!("isize: {}", std::any::type_name::<isize>());
    // array
    let array: [i64; 5];
    array = [3, 7, 31, 127, 8191];
    assert_eq!(array[0], 3);
    assert_eq!(array[4], 8191);
    let array = [57; 5];
    assert_eq!(array[0], 57);
    assert_eq!(array[4], 57);
    // pattern match
    let [x, y, z] = [1, 2, 3];
    assert_eq!(x, 1);
    assert_eq!(y, 2);
    assert_eq!(z, 3);
    // out of bounds
    let array = [0, 10, 20, 30, 40, 50];
    input! {
        index: usize,
    }
    let ans = array[index - 1];
    println!("{}", ans);

    /* Chapter 13: Format output */
    println!("{0} {0} {1} {1}", -2, 10);
    println!("{hoge} {hoge} {fuga} {fuga}", hoge = -2, fuga = 10);
    // format
    println!("{:6}", 79);
    println!("{:06}", 79);
    println!("{:^6}", 79);
    // debug output
    let tuple = (10_i32, 20_i32, 30_i32);
    println!("{:?}", tuple);
    println!("{:#?}", tuple);

    /* Chapter 14: Reference and lifetime */
    // reference
    let hoge: i8 = 100;
    println!("{:p}", &hoge);
    let reference: &i8 = &hoge;
    println!("{:p}", reference);
    println!("{:p}", &reference);
    //let reference: &u8 = &hoge;
    // dereferencing
    let hoge: i8 = 100;
    let reference = &hoge;
    assert_eq!(*reference, 100_i8);
    // dereferencing by operator
    let hoge: i8 = 100;
    let reference = &hoge;
    assert_eq!(reference + 1_i8, 101_i8);
    // dereferencing by println!
    let hoge: i8 = 100;
    let reference = &hoge;
    println!("{}", reference);
    // dereferencing by .
    let tuple: (i32, f64) = (10, 3.14);
    assert_eq!(tuple.0, 10_i32);
    assert_eq!(tuple.1, 3.14_f64);
    let reference = &tuple;
    assert_eq!(reference.0, 10_i32);
    assert_eq!(reference.1, 3.14_f64);
    // pattern match
    let hoge = 10;
    let reference = &hoge;
    let &copied = reference;
    assert_eq!(copied, 10);
    println!("hoge:   {:p}", &hoge);
    println!("copied: {:p}", &copied);
    // shadowing
    let hoge = 10;
    let reference = &hoge;
    let hoge = 20;
    assert_eq!(hoge, 20);
    assert_eq!(*reference, 10);
    // borrowing and dereferencing constraints
    let reference;
    {
        let hoge = 100;
        reference = &hoge;
        println!("{:p}", reference);
    }
    // println!("{}", *reference); // borrowed value does not live long enough
    // static lifetime
    let reference;
    {
        reference = &100;
    }
    assert_eq!(*reference, 100);

    /* Chapter 15: for expression */
    let primes = [2, 3, 5, 7];
    for p in &primes {
        println!("{}", p);
    }
}
