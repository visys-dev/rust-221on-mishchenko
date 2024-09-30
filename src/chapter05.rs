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

#[test]
fn test_ownership_07() {
    let x = Box::new(5);

    let mut y = Box::new(6); // update this line, don't change other lines!

    *y = 4;

    assert_eq!(*x, 5);

    println!("Success! test_ownership_07");
}

#[test]
fn test_ownership_08() {
    let t = (String::from("hello"), String::from("world"));

    let _s = t.0;

    // Modify this line only, don't use `_s`
    println!("{:?}", t.1);
}

#[test]

fn test_ownership_09() {
    let t = (String::from("hello"), String::from("world"));

    // Fill the blanks
    let (s1, s2) = t.clone();

    println!("{:?}, {:?}, {:?}", s1, s2, t); // -> "hello", "world", ("hello", "world")
}

//Reference and Borrowing
#[test]
fn test_borrowing_01() {
    let x = 5;
    // Fill the blank
    let p = &x;

    println!("the memory address of x is {:p} test_borrowing_01", p); // One possible output: 0x16fa3ac84
}

#[test]
fn test_borrowing_02() {
    let x = 5;
    let y = &x;

    // Modify this line only
    assert_eq!(5, *y);

    println!("Success! test_borrowing_01");
}

#[test]
// Fix error
fn test_borrowing_03() {
    let mut s = String::from("hello, ");

    borrow_object_test_03(&s);

    println!("Success! test_borrowing_03");
}

fn borrow_object_test_03(s: &String) {}