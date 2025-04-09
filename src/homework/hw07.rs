fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect()
}

fn main() {
    let test_str = "Hello, World!";
    println!("Original: {}", test_str);
    println!("Inverted: {}", invert_the_case(test_str.to_string()));
}
