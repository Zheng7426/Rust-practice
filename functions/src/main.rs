// use std::io;


// fn main() {
//     let mut input = String::new();
//     println!("Hello, world!");
//     println!("Enter a number!");
//     io::stdin()
//         .read_line(&mut input)
//         .expect("Failed to read input!");
//     let input: u32 = input.trim().parse().expect("Not good");
//     another_function(input);
// }

// fn another_function(x: u32) {
//     println!("Yo wasup! The value of x doubled is {}", x * 2);
// }
fn main() {
    let s = String::from("hello"); // s 进入作用域

    takes_ownership(s);            // s 的值进入函数，并且在此处不再有效

    let x = 5;                     // x 进入作用域

    makes_copy(x);                 // x 会进入函数，但 i32 类型是 Copy
                                   // 即没有堆数据的问题，全部数据在栈中
                                   // 所以 x 之后仍可以被使用 

} // 此处，x 出了作用域，然后是 s， s 的值被移动了。


fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 此处，some_string 出了作用域，并且 `drop` 被调用，其所占内存被释放。

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 此处，some_integer 出作用域。
