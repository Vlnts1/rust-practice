// Функція для перевірки, чи є число паліндромом
fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut reversed = 0;
    let mut number = x;

    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }

    original == reversed
}

// Тестова функція
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_palindrome() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        // Тестуємо кожен елемент
        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}