fn main() {
    const MAX_POINTS: u32 = 100_000;
    // Making x mutable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    // Shadowing
    let y = 5;
    let y = y + 1;

    let space = "   ";
    let space = space.len();
    println!("The value of y is: {}", y);
    println!("The value of space is: {}", space);
}
