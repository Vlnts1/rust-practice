use std::io;

fn simple_array_sum(ar: Vec<i32>) -> i32 {
    ar.iter().sum()
}

fn main() {
    let mut input = String::new();

    // Зчитуємо розмір масиву
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let _n: usize = input.trim().parse().expect("Invalid input");

    input.clear();

    // Зчитуємо сам масив
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let ar: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().expect("Not a number"))
        .collect();

    let result = simple_array_sum(ar);
    println!("{}", result);
}
