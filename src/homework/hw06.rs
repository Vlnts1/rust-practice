// Функція для малювання ялинки
fn draw_tree(triangles: u32) {
    // Ітерація для кожного трикутника
    for i in 1..=triangles {
        // Кількість рядків для поточного трикутника
        for row in 1..=i {
            let stars = 2 * row - 1;  // Кількість зірочок для поточного рядка
            let spaces = (2 * triangles - 1) - stars;  // Кількість пробілів для вирівнювання

            // Виводимо пробіли перед зірочками
            print!("{}", " ".repeat((spaces / 2) as usize));
            // Виводимо зірочки
            println!("{}", "*".repeat(stars as usize));
        }
    }
    // Малюємо стовпчик ялинки
    println!("{}", " ".repeat((triangles - 1) as usize) + "*");
}

fn main() {
    // Кількість трикутників у ялинці
    let triangles = 5;
    draw_tree(triangles);
}
