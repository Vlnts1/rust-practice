const SIZE: usize = 7; // Розмір ромба (повинен бути непарним)

fn main() {
    // Малюємо верхню частину ромба
    for i in 0..SIZE {
        for j in 0..SIZE - i - 1 {
            print!(" "); // Пробіли для вирівнювання
        }
        for j in 0..2 * i + 1 {
            print!("*"); // Зірочки для малювання ромба
        }
        println!(); // Перехід на новий рядок після кожного рівня
    }

    // Малюємо нижню частину ромба
    for i in (0..SIZE - 1).rev() {
        for j in 0..SIZE - i - 1 {
            print!(" "); // Пробіли для вирівнювання
        }
        for j in 0..2 * i + 1 {
            print!("*"); // Зірочки для малювання ромба
        }
        println!(); // Перехід на новий рядок після кожного рівня
    }
}
