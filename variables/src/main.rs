fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    const MAX_POINTS: u32 = 100_000;
    println!("Maximum points set is {}", MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 10;

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The spaces variable contains {}", spaces);

    let x: i32 = 1;
    println!("The value of x is: {}", x);
    let x = x << 1;
    println!("The value of x is: {}", x);
    let x = x << 1;
    println!("The value of x is: {}", x);
    let x = x >> 1;
    println!("The value of x is: {}", x);
    let x = x >> 1;
    println!("The value of x is: {}", x);
    let x = x >> 1;
    println!("The value of x is: {}", x);
    let x: i32 = -65535 >> 1;
    println!("The value of x is: {}", x);
    let x: i32 = -65535 << 5;
    println!("The value of x is: {}", x);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{} {} {}", c, z, heart_eyed_cat);

    let some_tup: (i32, f64, u8) = (65535, 3.14, 2);

    let (x, y, z) = some_tup;

    println!("{} {} {}", x, y, z);

    let some_arr: [u8; 5] = [1, 2, 3, 4, 5];

    for element in some_arr.iter() {
        println!("{}", element);
    }

    some_function();

    println!("{}", return_val_func(100));

    for i in 1..16 {
        fizz_buzz(i);
    }
}

fn some_function() {
    println!("Hello! This is a function.");
}

fn return_val_func(x: i32) -> i32 {
    x + 10
}

fn fizz_buzz(x: i64) {
    match (x % 3, x % 5) {
        (0, 0) => println!("FizzBuzz"),
        (0, _) => println!("Fizz"),
        (_, 0) => println!("Buzz"),
        _ => println!("{}", x)
    }
}
