// Практична робота №8 (Invert the case)
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

#[test]
fn test_practice_08() {
    let data = [
        ("Hello", "hELLO"),
        ("Привет", "пРИВЕТ"),
        ("World", "wORLD"),
        ("WORLD", "world"),
    ];

    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}
