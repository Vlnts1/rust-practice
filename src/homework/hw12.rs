// src/homeworks/hw12.rs

use rand::Rng;

/// Функція рахує мінімальну кількість переміщень вантажу,
/// щоб на всіх кораблях був однаковий вантаж.
/// Якщо це неможливо — повертає -1.
pub fn count_permutation(shipments: &Vec<u32>) -> isize {
    let n = shipments.len() as u32;
    let total: u32 = shipments.iter().sum();

    if total % n != 0 {
        return -1; // Рівномірний розподіл неможливий
    }

    let avg = total / n;
    let mut moves = 0;

    for &load in shipments {
        if load > avg {
            moves += (load - avg) as usize;
        }
    }

    moves as isize
}

/// Генерує вектор з n елементів, сума яких кратна n,
/// щоб розподіл вантажу був можливим.
pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..=50); // середня вага
    let mut shipments = vec![avg; n];

    for _ in 0..(n / 2) {
        let idx1 = rng.gen_range(0..n);
        let idx2 = rng.gen_range(0..n);
        let delta = rng.gen_range(1..=avg.min(10));
        if shipments[idx1] >= delta {
            shipments[idx1] -= delta;
            shipments[idx2] += delta;
        }
    }

    shipments
}

fn main() {
    // ✅ Тест 1: приклад з умови задачі
    let shipments1 = vec![1, 1, 1, 1, 6];
    println!("Test 1: {:?}", shipments1);
    println!("Moves needed: {}", count_permutation(&shipments1)); // Очікується: 4

    // ✅ Тест 2: всі кораблі вже рівномірно завантажені
    let shipments2 = vec![3, 3, 3];
    println!("\nTest 2: {:?}", shipments2);
    println!("Moves needed: {}", count_permutation(&shipments2)); // Очікується: 0

    // ❌ Тест 3: неможливо зробити розподіл
    let shipments3 = vec![1, 2, 3];
    println!("\nTest 3: {:?}", shipments3);
    println!("Moves needed: {}", count_permutation(&shipments3)); // Очікується: -1

    // ✅ Тест 4: автоматично згенеровані вхідні дані
    let generated = gen_shipments(5);
    println!("\nTest 4: {:?}", generated);
    println!("Moves needed: {}", count_permutation(&generated));
}
