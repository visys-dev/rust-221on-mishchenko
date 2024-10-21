// Практична робота №9 (Prime number)

fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_prime() {
        let test_values = [1, 2, 3, 4, 5, 29, 30, 31, 37, 42, 43, 45];

        for &value in test_values.iter() {
            let result = is_prime(value);
            println!("Число {} є простим: {}", value, result);
            assert_eq!(result, is_prime(value));
        }
    }
}

