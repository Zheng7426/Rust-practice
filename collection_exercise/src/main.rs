
fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

fn median(numbers: &mut [i32]) -> i32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

fn main() {
    let mut v = [5, 12, 7, 40, 35, 26, 5];
    println!("Average: {}", average(&v));
    println!("Median: {}", median(&mut v));
}
