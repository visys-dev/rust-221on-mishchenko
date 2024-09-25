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
