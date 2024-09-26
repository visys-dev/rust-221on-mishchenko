#[test]
// Remove something to make it work
fn test() {
    let x: i32 = 5;
    let mut y: u32 = 5;

    y = x as u32; // Декларуємо x, як u32

    let z = 10; // Type of z ?

    println!("Success!");
    println!("Type of {} - 'i32'", z);
}

#[test]
// Fill the blank
fn test2() {
    let v: u16 = 38_u8 as u16;

    println!("Success!");
}

#[test]
// Modify `assert_eq!` to make it work
fn test3() {
    let x = 5;
    assert_eq!("i32".to_string(), type_of(&x)); //Замінюємо "u32" на "i32"

    println!("Success!");
}

// Get the type of given variable, return a string representation of the type  , e.g "i8", "u8", "i32", "u32"
fn type_of<T>(_: &T) -> String {
    format!("{}", std::any::type_name::<T>())
}

#[test]
// Fill the blanks to make it work
fn test4() {
    assert_eq!(i8::MAX, 127);
    assert_eq!(u8::MAX, 255);

    println!("Success!");
}

#[test]
// Fix errors and panics to make it work
fn test5() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{} it's a MAX of u8, {} it's a MAX of i8", v1, v2);
}

#[test]
// Modify `assert!` to make it work
fn test6() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //1_024 = 1024, 0xff = 255, 0o77 = 63, 0b1111_1111 = 255
    assert!(v == 1597);

    println!("Success!");
}

// #[test]
// // Fill the blank to make it work
// fn test7() {
//     let x = 1_000.000_1; // f64
//     let y: f32 = 0.12; // f32
//     let z = 0.01_f64; // f64
//
//     assert_eq!(type_of(&x), "f64".to_string());
//     println!("Success!");
// }
//
// fn type_of<T>(_: &T) -> String {
//     format!("{}", std::any::type_name::<T>())
// }
