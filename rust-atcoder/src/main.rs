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

    /* Chapter 16: pattern matching */
    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for &(number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }
    // ref pattern
    let hoge = 10;
    let ref reference = hoge;
    assert_eq!(*reference, 10);

    let elements: [(i32, f64); 5] = [(6, 12.0), (7, 14.0), (8, 16.0), (15, 31.0), (16, 32.1)];
    for (number, weight) in &elements {
        println!("{}: {:.1}", number, weight);
    }

    /* Chapter 17: Mutable variable */
    let mut mutable: i32;
    mutable = 30;
    //mutable = 20_f64;
    assert_eq!(mutable, 30);
    let (mut hoge, fuga) = (10, 20);
    assert_eq!(hoge, 10);
    hoge += 5;
    assert_eq!(hoge, 15);
    assert_eq!(fuga, 20);
    mutable += 20;
    assert_eq!(mutable, 50);
    // for + mutable variable
    let array = [30, 20, 30];
    let mut sum = 0;
    for num in &array {
        sum += num;
    }
    assert_eq!(sum, 80);

    /* Chapter 18: Mutable reference */
    // reference to mutable variable
    let mut hoge = 10;
    assert_eq!(hoge, 10);
    hoge = 20;
    assert_eq!(hoge, 20);
    let reference = &hoge;
    println!("{}", reference);
    //*reference = 20;
    // lifetime and variability
    hoge = 30;
    assert_eq!(hoge, 30);
    //println!("{}", reference);
    // multiple usage
    let mut hoge = 10;
    let reference1 = &hoge;
    let reference2 = &hoge;
    assert_eq!(*reference1, 10);
    assert_eq!(*reference2, 10);
    hoge = 20;
    assert_eq!(hoge, 20);
    // variable variables of reference type
    let hoge = 10;
    let fuga = 20;
    let mut reference = &hoge;
    println!("{:p}", reference);
    assert_eq!(*reference, 10);
    reference = &fuga;
    println!("{:p}", reference);
    assert_eq!(*reference, 20);
    // variable reference
    let mut hoge = 10;
    let reference = &mut hoge;
    assert_eq!(*reference, 10);
    *reference = 20;
    assert_eq!(*reference, 20);
    let immutable_refecence = &hoge;
    // error[E0502]: cannot borrow `hoge` as mutable because it is also borrowed as immutable
    // let mutable_reference = &mut hoge;
    println!("{}", immutable_refecence);
    // use of variables borrowed as variable.
    let mut hoge = 10;
    let mutable_reference = &mut hoge;
    *mutable_reference += 30;
    let immutable_reference = &hoge;
    let fuga = immutable_reference + 20;
    assert_eq!(fuga, 60);
    // pattern match
    let mut hoge = 10;
    let &mut copied = &mut hoge;
    //let &mut copied = &hoge;
    //let &copied = &mut hoge;
    assert_eq!(copied, 10);
    // for expression
    let mut array = [10, 20, 30];
    for i in &mut array {
        *i += 1;
    }
    assert_eq!(array, [11, 21, 31]);

    /* Chapter 19: Various loops */
    // break
    let array = [2, 3, 0, 4, 5];
    for &i in &array {
        if i == 0 {
            break;
        }
        print!("{}, ", i);
    }
    println!("end");
    // continue
    let array = [2, 3, 0, 4, 5];
    for &i in &array {
        if i == 0 {
            continue;
        }
        print!("{}, ", i);
    }
    println!("end");
    // loop
    loop {
        input! {
            x: i32,
        }
        if x > 0 {
            println!("{}", x * 2);
        } else {
            break;
        }
    }
    // while
    let mut x = 120;
    while x % 2 == 0 {
        println!("{}", x);
        x /= 2;
    }
    assert_eq!(x, 15);
    // ..
    for i in 0..5 {
        println!("{}", i);
    }
    for i in 0..=5 {
        println!("{}", i);
    }
    for i in 3.. {
        println!("{}", i);
        if i * i > 30 {
            break;
        }
    }

    /* Chapter 20: Loop labels and value returns */
    // double loop and label
    'outer: for i in 0..4 {
        for j in 0..i {
            if i * j >= 2 {
                println!();
                break 'outer;
            }
            print!("({}, {}) ", i, j);
        }
        println!();
    }
    // loop expression that returns a value
    let value = loop {
        input! {
            x: i32,
        }
        if x > 0 {
            println!("{}", x * 2);
        } else {
            break x;
        }
    };
    println!("value: {}", value);
    // lebel
    let factor = 'input: loop {
        input! {
            x: i32,
        }
        for i in 2.. {
            if i * i > x {
                break;
            } else if x % i == 0 {
                break 'input i;
            }
        }
    };
    println!("{}", factor);

    /* Chapter 21: Function */
    let value = {
        let mut n = 1;
        for i in 1..=5 {
            print!("{} ", n);
            n *= i;
        }
        println!("{}", n);
        n
    };
    assert_eq!(value, 120);
    assert_eq!(fact5(), 120);
    assert_eq!(prod(1, 5), 120);
    assert_eq!(prod(3, 6), 360);
    let tuple = (5, 10);
    assert_eq!(swap(tuple), (10, 5));
    let var = 5;
    assert_eq!(double(var), 25);
    assert_eq!(var, 5);
    assert_eq!(minimum_factor(2021), 43);
    assert_eq!(minimum_factor(43), 43);

    /* Chapter 22: Crate and pass */
    let x: i32 = rand::random();
    println!("{}", x);
    assert_eq!(std::cmp::max(2, 5), 5);
    assert_eq!(std::cmp::min(2, 5), 2);
    let x = 10;
    let y = 10;
    let z = std::cmp::max(&x, &y);
    println!("&x: {:p}", &x);
    println!("&y: {:p}", &y);
    println!("z: {:p}", z);

    /* Chapter 23: Recursive function */
    assert_eq!(fact(3), 6);
    assert_eq!(digit_sum(6318), 18);
    assert_eq!(gcd(18, 30), 6);
    assert_eq!(gcd(30, 18), 6);
    assert_eq!(gcd(15, 24), 3);

    /* Chapter 24: Vector */
    // initialize
    let x = vec![1, 2, 3];
    println!("{:?}", x);
    let x = vec![1; 10];
    println!("{:?}", x);
    let x = Vec::<i32>::new();
    println!("{:?}", x);
    // length
    let vector: Vec<i32> = vec![1, 2, 3];
    assert_eq!(vector.len(), 3_usize);
    assert_eq!(vector[0], 1_i32);
    assert_eq!(vector[1], 2_i32);
    assert_eq!(vector[2], 3_i32);
    let mut vector: Vec<i32> = vec![1, 2, 3];
    vector[1] = 10;
    assert_eq!(vector, vec![1, 10, 3]);
    // add values
    let mut vector: Vec<u32>;
    vector = Vec::new();
    println!("{:?}", vector);
    vector.push(10);
    vector.push(20);
    vector.push(30);
    assert_eq!(vector, vec![10, 20, 30]);
    vector.pop();
    assert_eq!(vector, vec![10, 20]);
    vector.pop();
    vector.pop();
    assert_eq!(vector, vec![]);
    // proconio::input!
    input! {
        vector: [i32; 2usize],
    }
    println!("{} {}", vector[0], vector[1]);
    input! {
        n: usize,
        vector: [i32; n],
    }
    println!("{:?}", vector);
    // for
    let vector = vec![30, 20, 30];
    let mut sum = 0;
    for num in &vector {
        sum += num;
    }
    assert_eq!(sum, 80);

    /* Chapter 25: Ownership */
    // ownership
    let ref_elem;
    {
        let vector = vec![10, 20, 30];
        {
            let ref_entire = &vector;
            println!("{:?}", ref_entire);
        }
        ref_elem = &vector[1];
        println!("{:?}", ref_elem);
    }
    // assert_eq!(*ref_elem, 20); // `vector` does not live long enough
    // move
    let vector = vec![10, 20, 30];
    let _moved = vector;
    // println!("{:?}", vector); // borrow of moved value: `vector`
    // copyable type
    let value = 10;
    let copied = value;
    assert_eq!(value, copied);
    let digits = {
        let mut tmp = Vec::new();
        for i in 0..10 {
            tmp.push(i);
        }
        tmp
    };
    println!("{:?}", digits);
    let test_scores = vec![82, 91, 79];
    let total_score = sums(test_scores);
    // assert_eq!(test_scores[0], 82); // borrow of moved value: `test_scores`
    assert_eq!(total_score, 252);
    // tuple
    let tuple: (Vec<i32>, i32) = (vec![], 7);
    let _moved = tuple;
    // assert_eq!(tuple.1, 7); // borrow of moved value: `tuple`
    let vector = vec![];
    let _tuple: (Vec<i32>, i32) = (vector, 10);
    // println!("{:?}", vector); // borrow of moved value: `vector`
    let tuple = (vec![10, 20, 30], 7);
    let _vector = tuple.0;
    // assert_eq!(tuple.0[0], 10); // borrow of moved value: `tuple.0`
    assert_eq!(tuple.1, 7);

    /* Chapter 26: Slice */
    // slice
    // let slice: [i32];
    let _ref_slice: &[i32];
    // type force
    let mut ref_slice: &[i32];
    let array = [1, 2, 3];
    ref_slice = &array;
    println!("{:?}", ref_slice);
    let vector = vec![4, 5, 6];
    ref_slice = &vector;
    println!("{:?}", ref_slice);
    // usage
    let ref_slice: &[i32] = &[50, 20, 30];
    assert_eq!(ref_slice[0], 50);
    assert_eq!(ref_slice[1], 20);
    assert_eq!(ref_slice[2], 30);
    // ref_slice[3]; // index out of bounds: the len is 3 but the index is 3
    let ref_slice: &[i32] = &[50, 20, 30];
    for &i in ref_slice {
        println!("{}", i);
    }
    // partial slice
    let array = [0, 10, 20, 30, 40, 50];
    let ref_slice = &array[1..4];
    println!("{:?}", ref_slice);
    let ref_slice = &array[1..=4];
    assert_eq!(ref_slice, [10, 20, 30, 40]);
    let ref_slice = &array[1..];
    assert_eq!(ref_slice, [10, 20, 30, 40, 50]);
    let ref_slice = &array[..4];
    assert_eq!(ref_slice, [0, 10, 20, 30]);
    let ref_slice = &array[..];
    assert_eq!(ref_slice, [0, 10, 20, 30, 40, 50]);
    // empty slice
    let empty = &[1, 2, 3, 4, 5][2..2];
    println!("{:?}", empty);
    // method
    let mut array = [0, 10, 20, 30, 40];
    let ref_mut_slice = &mut array[..];
    ref_mut_slice.swap(1, 3);
    assert_eq!(array, [0, 30, 20, 10, 40]);
    let mut array = [0, 10, 20, 30, 40];
    array.swap(1, 3);
    assert_eq!(array, [0, 30, 20, 10, 40]);
    let mut array = [7, 2, -3, 9, -2, 5];
    array.reverse();
    assert_eq!(array, [5, -2, 9, -3, 2, 7]);
    let mut array = [7, 2, -3, 9, -2, 5];
    array[1..4].reverse();
    assert_eq!(array, [7, 9, -3, 2, -2, 5]);
    let mut array = [7, 2, 5, -3, 9, -2, 5];
    array.sort();
    assert_eq!(array, [-3, -2, 2, 5, 5, 7, 9]);

    /* Chapter 27: Call by reference  */
    // arguments and moves
    let vector = vec![20, 80, 60, 40];
    let s = sumv(vector);
    assert_eq!(s, 200);
    // println!("{:?}", vector); // borrow of moved value: `vector`
    // call by reference
    let vector = vec![20, 80, 60, 40];
    let s: i32 = sumr(&vector);
    assert_eq!(s, 200);
    println!("{:?}", vector);
    // mutable reference
    let mut hoge = 10;
    double2(&mut hoge);
    assert_eq!(hoge, 20);
    double2(&mut hoge);
    assert_eq!(hoge, 40);
    // array and vector borrowing
    let mut x = 20;
    let mut y = 30;
    std::mem::swap(&mut x, &mut y);
    assert_eq!(x, 30);
    assert_eq!(y, 20);
    let mut array = [1, 2, 3, 4, 5];
    // std::mem::swap(&mut array[0], &mut array[1]); // cannot borrow `array[_]` as mutable more than once at a time
    array.swap(0, 1);
    println!("{:?}", array);
    // dbg!
    let mut x = 0;
    for i in 18..=20 {
        x += i;
        dbg!(x);
    }
    println!("{}", x);

    /* Chapter 28: Pattern Matching and Conditional Branching  */
    // irrefutable patterns
    let ref_slice: &[i32] = &[10, 15, 20];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("Failed to match");
    }
    let ref_slice: &[i32] = &[10, 15];
    if let [x, y, z] = *ref_slice {
        println!("{} {} {}", x, y, z);
    } else {
        println!("Failed to match");
    }
    // literal pattern
    input! {
        vector: [(i32, i32); 5],
    }
    for &tuple in &vector {
        if let (1, value) = tuple {
            println!("{}", value);
        } else if let (2, value) = tuple {
            println!("{}", value * value);
        } else if let (0, 0) = tuple {
            break;
        } else {
            println!("?");
        }
    }
    // multiple patterns
    let array = [(1, 92), (3, 91), (95, 1), (94, 2)];
    let mut vector = Vec::new();
    for tuple in &array {
        if let (1, value) | (value, 2) = *tuple {
            vector.push(value);
        }
    }
    assert_eq!(vector, vec![92, 94]);
    let _tuple = (3, 2, 1);
    //if let (x, 0, 0) | (x, y, 1) | (x, y, 2) = _tuple { // variable `y` is not bound in all patterns
    let _tuple: (i32, f64) = (1, 2.0);
    // if let (1, x) | (x, 2.0) = _tuple { // mismatched types a binding must have the same type in all alternatives
    let tuple = (1, 2);
    if let (0..=5, x) = tuple {
        assert_eq!(x, 2);
    } else {
        panic!();
    }
    // wildcard pattern
    let tuple = (3, 1, 2);
    if let (1, _, _) | (_, 1, _) | (_, _, 1) = tuple {
        println!("At least one of them is 1");
    }
    for _ in 0..4 {
        println!("Knock, knock, knockin' on heaven's door");
    }
    let array = [0, 0, 0, 1, 2];
    let mut ref_slice = &array[..];
    while let [0, ..] = *ref_slice {
        ref_slice = &ref_slice[1..];
    }
    println!("{:?}", ref_slice);
    // match expression
    input! {
        vector: [(i32, i32); 5],
    }
    for &tuple in &vector {
        match tuple {
            (1, value) => println!("{}", value),
            (2, value) => println!("{}", value * value),
            (0, 0) => break,
            _ => println!("?"),
        }
    }
    // match expression returns value
    input! {
        x: i32,
    }
    let y = match x {
        0 => 1,
        1 => 0,
        _ => {
            println!("neither 0 nor 1");
            0
        }
    };
    println!("{}", y);
    input! {
        x: i32,
    }
    match x % 3 {
        0 => println!("3x"),
        1 | -2 => println!("3x + 1"),
        2 | -1 => println!("3x - 1"),
        _ => unreachable!(),
    }
    // the value the match expression returns
    loop {
        input! {
            x: i32,
        }
        let y = match x {
            0 => break,
            x => x - 1,
        };
        println!("{}", y);
    }
    // match guard
    let tuple = (1, 3);
    match tuple {
        (1, x) if x % 2 == 0 => println!("{}", x),
        _ => {}
    }
    let tuples = [(2, 5), (4, 4), (1, -4), (-3, -3)];
    let mut vector = Vec::new();
    for &tuple in &tuples {
        match tuple {
            (x, y) if x == y => vector.push(x),
            _ => {}
        }
    }
    println!("{:?}", vector);

    /* Chapter 28: Character string */
    let _string = "Hello, world!";
    let s = String::new();
    let _slice: &str = &s;
    let _string = "Hello".to_string();
    let _string = String::from("Hello");
    let greeting = "Hello";
    let world = "world".to_string();
    println!("{}, {}!", greeting, world);
    // for expression
    let s = "打打打打打打打打打打";
    let da: char = '打';
    for c in s.chars() {
        assert_eq!(c, da);
    }
    let s = "𠮷野家で𩸽";
    for c in s.bytes() {
        println!("{:x}", c);
    }
    // conversion from non-string to String
    let x: i32 = 10;
    assert_eq!(x.to_string(), "10".to_string());
    let x: f64 = 120.0;
    assert_eq!(x.to_string(), "120".to_string());
    let x: char = 'A';
    assert_eq!(x.to_string(), "A".to_string());
    let s = format!("{} {}", 10, 2.5);
    println!("{}", s);
    // byte literal
    let c = b'A';
    let d = 'A';
    println!("{:x} {}", c, d);
    let array = b"Hello";
    assert_eq!(*array, [b'H', b'e', b'l', b'l', b'o']);
    // proconio::input!
    input! {
        s1: String,
        s2: String,
    }
    println!(r#""{}""#, s1);
    println!(r#""{}""#, s2);
    input! {
        s: proconio::marker::Chars,
    }
    for c in &s {
        println!("{}", c);
    }
    input! {
        s: proconio::marker::Bytes,
    }
    for c in &s {
        println!("{}", c);
    }
    input! {
        c1: char,
        c2: char,
        c3: char,
    }
    println!("{} {} {}", c1, c2, c3);

    /* Chapter 30: Operators and bool types */
    assert_eq!(std::ops::Add::add(2, 3), 5);
    assert_eq!(std::ops::Sub::sub(5, 2), 3);
    assert_eq!(std::ops::Mul::mul(3, 4), 12);
    assert_eq!(std::ops::Div::div(14, 3), 4);
    assert_eq!(std::ops::Rem::rem(14, 3), 2);
    input! {
        x:i32,
    }
    if PartialEq::eq(&x, &5) {
        println!("x equals to 5.");
    }
    for i in 0..=100 {
        if is_prime(i) {
            println!("{}", i);
        }
    }
    let result = fnc1() & fnc2();
    assert_eq!(result, false);
    let result = fnc1() && fnc2();
    assert_eq!(result, false);
    input! {
        x: i32,
    }
    //if (x != 0) & (12 % x == 0) {
    if (x != 0) && (12 % x == 0) {
        println!("{} is divisor of 12.", x);
    }
    let result = fnc1() | fnc2();
    assert_eq!(result, true);
    let result = fnc2() || fnc1();
    assert_eq!(result, true);
    assert_eq!(true ^ true, false);
    assert_eq!(true ^ false, true);
    assert_eq!(false ^ true, true);
    assert_eq!(false ^ false, false);
    assert_eq!(!true, false);
    assert_eq!(!false, true);
}

fn fact5() -> i32 {
    let mut n = 1;
    for i in 1..=5 {
        print!("{} ", n);
        n *= i;
    }
    println!("{}", n);
    n
}

fn prod(a: i32, b: i32) -> i32 {
    let mut n = 1;
    for i in a..=b {
        n *= i;
    }
    n
}

fn swap((a, b): (i32, i32)) -> (i32, i32) {
    (b, a)
}

fn double(mut x: i32) -> i32 {
    x *= x;
    x
}

fn minimum_factor(n: i32) -> i32 {
    for i in 2.. {
        if i * i > n {
            break;
        } else if n % i == 0 {
            return i;
        }
    }
    n
}

fn fact(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        fact(n - 1) * n
    }
}

fn digit_sum(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    let last_digit = n % 10;
    let higher_digits_sum = digit_sum(n / 10);
    let result = higher_digits_sum + last_digit;
    println!("{} -> {} + {}", n, higher_digits_sum, last_digit);
    result
}

fn gcd(m: i32, n: i32) -> i32 {
    if n == 0 {
        return m;
    } else {
        gcd(n, m % n)
    }
}

fn sums(vector: Vec<i32>) -> i32 {
    let mut ret = 0;
    for elem in &vector {
        ret += elem;
    }
    ret
}

fn sumv(v: Vec<i32>) -> i32 {
    let mut ret = 0;
    for &i in &v {
        ret += i;
    }
    ret
}

fn sumr(v: &Vec<i32>) -> i32 {
    let mut ret = 0;
    for &i in v {
        ret += i;
    }
    ret
}

fn double2(x: &mut i32) {
    *x *= 2;
}

fn is_prime(x: i32) -> bool {
    if x < 2 {
        return false;
    }
    for i in 2.. {
        if i * i > x {
            return true;
        }
        if x % i == 0 {
            return false;
        }
    }
    unreachable!();
}

fn fnc1() -> bool {
    println!("fnc1: false");
    false
}

fn fnc2() -> bool {
    println!("fnc2: true");
    true
}
