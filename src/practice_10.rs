// Практична робота №10 (String rotate)

fn rotate(s: String, n: isize) -> String {
    let len = s.len();
    let effective_shift = ((n % len as isize) + len as isize) % len as isize;
    let effective_shift = effective_shift as usize;

    let rotated = [&s[len - effective_shift..], &s[..len - effective_shift]].concat();
    rotated
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let s = "abcdefgh";
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        for (n, expected) in shifts.iter() {
            assert_eq!(rotate(s.to_string(), *n), expected.to_string());
        }
    }
}

#[test]
fn test_practice_10() {
    let result = rotate("abcdefgh".to_string(), 3);
    println!("Result: {}", result);
}
