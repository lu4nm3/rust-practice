fn main() {

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const MAX_POINT: u32 = 100_100;

    let spaces: &str = "   ";
    let spaces: usize = spaces.len();

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", tup.1);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let index = 10;
//    a[index];
}
