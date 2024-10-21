// Практична робота №11 (Palindromic number)

fn is_palindrome(num: i32) -> bool {
    let num_str = num.to_string();
    let reverse_str = num_str.chars().rev().collect::<String>();
    num_str == reverse_str
}

#[test]
fn test_practice_11() {
    let number = 123454321;
    if is_palindrome(number) {
        println!("{} is palindrome", number);
    } else {
        println!("{} is not palindrome", number);
    }
}
