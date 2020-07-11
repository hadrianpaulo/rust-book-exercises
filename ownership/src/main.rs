fn main() {
    // This is just a string literal.
    // These are immutable and hardcoded.
    let _s = "hello";

    // Strings are allocated in the heap.
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // y copies the value of x.
    // Both values are stored in the stack.
    // "Passed by value."
    let x = 5;
    let y = x;

    println!("{}", x);
    println!("{}", y);

    // s2 only gets the reference data copied from s1.
    // The underlying data is allocated in the heap.
    // It is not copied twice for s2.
    let s1 = String::from("hello");
    let s2 = s1;

    // s1 is now an invalidated reference due to ownership rules.
    // println!("{}, world!", s1);
    println!("{}, world!", s2);

    // To copy the underlying data from s1 to s2.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // Any variable that points to data that's allocated in the heap, when passed into a function without using references, will lose ownership.
    // s1 loses ownership of the underlying data but s2 is does not.
    print_noref(s1);
    print_ref(&s2);
    // println!("{}", s1);
    println!("{}", s2);

    // However, it's not possible to modify the underlying data of a reference unless marked as mutable.
    let mut s2 = s2.clone();
    modify_ref(&mut s2);
    println!("{}", s2);

    // You can only have one mutable reference of a variable in a scope.
    let s3 = &mut s2;
    // let s4 = &mut s2;
    println!("{}", s3);

    // String slices
    let s = String::from("Hello Rust!");

    let hello = &s[..5];
    let rust = &s[6..];
    let hello_rust = &s[..];

    println!("{} {}", hello, rust);
    println!("{}", hello_rust);

    word_split(&String::from("Hello! I'm learning Rust!"));
}

fn print_noref(s: String) {
    println!("{}", s);
}

fn print_ref(s: &String) {
    println!("{}", s);
}

fn modify_ref(s: &mut String) {
    s.push_str(", world!");
}

fn word_split(s: &String) {
    let bytes = s.as_bytes();
    let mut counter = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            println!("{}", &s[counter..i]);
            counter = i + 1;
        }
    }
    println!("{}", &s[counter..]);
}
