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

#[test]
// Fix error
fn test_borrowing_04() {
    let mut s = String::from("hello, ");

    push_str_test_04(&mut s);

    println!("Success!");
}

fn push_str_test_04(s: &mut String) {
    s.push_str("world")
}

#[test]
fn test_borrowing_05() {
    let mut s = String::from("hello, ");

    // Fill the blank to make it work
    let p = &mut s;

    p.push_str("world");

    println!("Success! test_borrowing_05");
}

#[test]
fn test_borrowing_06() {
    let c = '中';

    let r1 = &c;
    // Fill the blank，dont change other code
    let ref r2 = c; //ref - створення посилання на змінну c замість "&c"

    assert_eq!(r1, r2);

    // Check the equality of the two address strings
    assert_eq!(get_addr_borrowing_06(r1), get_addr_borrowing_06(r2));

    println!("Success! test_borrowing_06");
}

// Get memory address string
fn get_addr_borrowing_06(r: &char) -> String {
    format!("{:p}", r)
}

#[test]

// Remove something to make it work
// Don't remove a whole line !
fn test_borrowing_07() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;

    println!("{}, {}", r1, r2);

    println!("Success! test_borrowing_07");
}

#[test]

fn test_borrowing_08() {
    // Fix error by modifying this line
    let mut s = String::from("hello, ");

    borrow_object_borrowing_08(&mut s);

    println!("Success! test_borrowing_08");
}

fn borrow_object_borrowing_08(s: &mut String) {}

#[test]

// This code has no errors!
fn test_borrowing_09() {
    let mut s = String::from("hello, ");

    borrow_object_borrowing_09(&s);

    s.push_str("world");

    println!("Success! test_borrowing_09");
}

fn borrow_object_borrowing_09(s: &String) {}

#[test]

// Comment one line to make it work
fn test_borrowing_10() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    let r2 = &mut s;
    r2.push_str("!");

    // println!("{}", r1);
    // println!("{}", s);
    // println!("Success! test_borrowing_10");
    println!("{}\n Success! test_borrowing_10", s)
}


#[test]

fn test_borrowing_11() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add one line below to make a compiler error: cannot borrow `s` as mutable more than once at a time
    // You can't use r1 and r2 at the same time
    // r1.push_str("world"); - Зняти коментар щоб була помилка
    // println!("Success! test_borrowing_11");
}