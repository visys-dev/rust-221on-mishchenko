//Ownership
#[test]
fn test_ownership_01_01() {
    // Use as many approaches as you can to make it work
    let x = String::from("Hello world");
    let y = x.clone();
    println!("{}, {}, test_01_01 success", x, y);
}

#[test]
fn test_ownership_01_02() {
    // Use as many approaches as you can to make it work
    let x = "Hello world";
    let y = x;
    println!("{}, {}, test_01_02 success", x, y);
}

#[test]
fn test_ownership_01_03() {
    // Use as many approaches as you can to make it work
    let x = &String::from("Hello world");
    let y = x;
    println!("{}, {}, test_01_03 success", x, y);
}

#[test]
// Don't modify code in main!
fn test_ownership_02() {
    let s1 = String::from("Hello world");
    let s2 = take_ownership_test_02(s1);

    println!("{}, test_ownership_02 success ", s2);
}

// Only modify the code below!
fn take_ownership_test_02(s: String) -> String {
    println!("{}", s);
    s
}

#[test]
fn test_ownership_03() {
    let s = give_ownership_test_03();
    println!("{}, test_ownership_03 success", s);
}

// Only modify the code below!
fn give_ownership_test_03() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();
    s
}

#[test]
// Fix the error without removing any code
fn test_ownership_04() {
    let s = String::from("Hello World");

    print_str_test_04(&s);

    println!("{}, test_ownership_04 success", s);
}

fn print_str_test_04(s: &String) {
    println!("{}", s)
}

#[test]
// Don't use clone ,use copy instead
fn test_ownership_05() {
    let x = (1, 2, (), "hello"); // замість String, буде &str який підтримує copy
    let y = x;
    println!("{:?}, {:?}, test_test_ownership_05 success", x, y);
}

#[test]
// make the necessary variable mutable
fn test_ownership_06() {
    let s = String::from("Hello ");

    let mut s1 = s;

    s1.push_str("World!");

    println!("Success! test_ownership_06");
}

