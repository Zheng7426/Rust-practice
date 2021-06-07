use std::io;


fn main() {
    let mut input = String::new();
    println!("Hello, world!");
    println!("Enter a number!");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input!");
    let input: u32 = input.trim().parse().expect("Not good");
    another_function(input);
}

fn another_function(x: u32) {
    println!("Yo wasup! The value of x doubled is {}", x * 2);
}
