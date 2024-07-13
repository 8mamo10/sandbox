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
}
