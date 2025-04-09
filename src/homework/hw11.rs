use rand::Rng; // Для генерації випадкових чисел

// Генерує випадковий вектор довжиною n з чисел в діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Функція для пошуку мінімальної суми двох сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX; // Початкова мінімальна сума
    let mut min_idx1 = 0;
    let mut min_idx2 = 1;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_idx1 = i;
            min_idx2 = i + 1;
        }
    }

    (min_sum, min_idx1, min_idx2)
}

// Функція для виведення результату в консоль
fn print_result(data: Vec<i32>) {
    let (min_sum, idx1, idx2) = min_adjacent_sum(&data);

    // Виведення індексів та чисел
    println!("indexes: {}", (0..data.len()).map(|i| format!("{:>2}.", i)).collect::<String>());
    println!("data:  {:?}", data);
    let mut indexes_line = "indexes: ".to_string();
    for i in 0..data.len() {
        if i == idx1 {
            indexes_line.push_str("\\__ ");
        } else if i == idx2 {
            indexes_line.push_str(" __/ ");
        } else {
            indexes_line.push_str("    ");
        }
    }
    println!("{}", indexes_line.trim_end());
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}

fn main() {
    // Генеруємо випадкові вектори та виводимо результат
    let vectors = [
        gen_random_vector(20),
        gen_random_vector(20),
        gen_random_vector(20),
        gen_random_vector(20),
    ];

    for data in vectors.iter() {
        print_result(data.clone());
        println!(); // Для кращого відступу між тестами
    }
}
