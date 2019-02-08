fn main() {
    let s1 = String::from("hello");
//    let s2 = s1; // move s1 into s2

    println!("{}, world!", s1);


    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);



    let x = 5;
    // types like integers have known size at compile time and are stored entirely on the stack; so
    // copies of the actual values are quick to make and there is no reason we would want to prevent
    // x from being valid after we create y
    //
    // Rust has Copy trait we can place on types that are stored on stack
    let y = x;

    println!("x = {}, y = {}", x, y);



    // any group of simple scalar values can be Copy, including compount types if they only contain
    // types that are also Copy
    let arr = [1, 2, 3];
    let arr2 = arr;

    for n in arr2.iter() {
        println!("{}", n)
    }



    let s = String::from("hello");

    println!("s: {}", s);

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    println!("x: {}", x);



    let s1 = gives_ownership();
    let s11 = gives_ownership();

    let s2 = String::from("hello");

    let s3 = takes_and_gives_back(s2);



    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}", s2, len);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}