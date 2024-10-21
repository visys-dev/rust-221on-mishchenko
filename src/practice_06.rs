// Практична робота №6 (greatest common divisor)
fn gcd(a: u32, b: u32) -> u32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        let data = [
            ((24, 60), 12),
            ((15, 9), 3),
            ((15, 6), 3),
            ((140, 40), 20),
            ((24, 16), 8),
            ((100, 10), 10),
            ((120, 80), 40),
            ((80, 120), 40),
            ((100, 20), 20),
            ((37, 11), 1),
            ((120, 90), 30),
            ((770, 330), 110),
            ((48, 18), 6),
            ((103, 101), 1),
            ((48, 180), 12)
        ];

        for ((a, b), exp) in data.iter() {
            let result = gcd(*a, *b);
            assert_eq!(*exp, result);
            println!("gcd({}, {}) = {}, expected: {}", a, b, result, exp);
        }
    }
}
