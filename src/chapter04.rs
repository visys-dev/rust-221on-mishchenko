//Numbers
#[test]
// Remove something to make it work
fn test_numbers_01() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32; // Декларуємо x, як u32

    let z = 10; // Type of z ?

    println!("Success!");
    println!("Type of {} - 'i32'", z);
}

#[test]
// Fill the blank
fn test_numbers_02() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
// Modify `assert_eq!` to make it work
fn test_numbers_03() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of_03(&x)); //Замінюємо "u32" на "i32"

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of_03<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
// Fill the blanks to make it work
fn test_numbers_04() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
// Fix errors and panics to make it work
fn test_numbers_05() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{} it's a MAX of u8, {} it's a MAX of i8", v1, v2);
}

#[test]
// Modify `assert!` to make it work
fn test_numbers_06() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //1_024 = 1024, 0xff = 255, 0o77 = 63, 0b1111_1111 = 255
    assert!(v == 1597);

    println!("Success!");
}

#[test]
// Fill the blank to make it work
fn test_numbers_07() {
    let x = 1_000.000_1; // f64
    let y: f32 = 0.12; // f32
    let z = 0.01_f64; // f64

    assert_eq!(type_of_07(&x), "f64".to_string());
    println!("Success! test_numbers_07");
}

fn type_of_07<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
fn test_numbers_08_1() {
    assert!(0.1_f32 + 0.2_f32 == 0.3f32);

    println!("Success! test_numbers_08_1");
}

// TODO: Не працює
// #[test]
// fn test_numbers_08_2()
//     assert!(0.1_f64 + 0.2_f64 == 0.3_f64);
//
//     println!("Success! test_numbers_08_2");
// }

#[test]
fn test_numbers_09() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}

#[test]
// Fill the blanks
fn test_numbers_10() {
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range { start: 1, end: 5 }); // від 1 до 5 (не включає)
    assert_eq!((1..=5), RangeInclusive::new(1, 5)); // від 1 до 5 (включає)

    println!("Success!");
}

#[test]
// Fill the blanks and fix the errors
fn test_numbers_11() {
    // Integer addition
    assert!(1u32 + 2 == 3);

    // Integer subtraction
    assert!(1i32 - 2 == -1);
    assert!(1i8 - 2 == -1);

    assert!(3 * 50 == 150);

    assert!(9 / 3 == 3); // error ! make it work

    assert!(24 % 5 == 4);
    // Short-circuiting boolean logic
    assert!(true && false == false);
    assert!(true || false == true);
    assert!(!true == false);

    // Bitwise operations
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);
}

//Char
#[test]
// Make it work
fn test_char_01() {
    use std::mem::size_of_val;
    let c1 = 'a'; // ASCII символ
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中'; // Юні код символ
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}

#[test]
// Make it work
fn test_char_02() {
    let c1 = "中";
    print_char(c1);
}

fn print_char(c: &str) {
    //Заміна char на string
    println!("{}", c);
}

#[test]
// Make println! work
fn test_char_03() {
    let _f: bool = false;

    let t = false;
    if !t {
        println!("Success!");
    }
}

#[test]

// Make it work
fn test_char_04() {
    let f = true;
    let t = true || false;
    assert_eq!(t, f);

    println!("Success! test_char_03");
}

#[test]

// Make it work, don't modify `implicitly_ret_unit` !
fn test_char_05() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(_v, implicitly_ret_unit());

    println!("Success! test_char_05");
}

fn implicitly_ret_unit() {
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {
    println!("I will return a ()");
}

#[test]
// Modify `4` in assert to make it work
fn test_char_06() {
    use std::mem::size_of_val;
    let unit: () = ();
    assert!(size_of_val(&unit) == 0); // () об'єкт - не займає пам'ять

    println!("Success! test_char_06 ");
}
