fn main() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x += 1; // This line will cause a compile-time error because x is immutable
    println!("The value of x is now: {}", x);
}
