//Compound Types
//String

#[test]
// Fix error without adding new line
fn test_string_01() {
    let s: &str = "hello, world";

    println!("{}\n Success! test_string_01", s);
}

#[test]

// Fix the error with at least two solutions
fn test_string_02() {
    let s: Box<str> = "hello, world".into();
    greetings_string_02(&s); // Або посилання через "&" або розпакувати Box через "*s"
    println!("Success! test_string_02");
}

fn greetings_string_02(s: &str) {
    println!("{}", s)
}

#[test]

// Fill the blank
fn test_string_03() {
    let mut s = String::new();
    s.push_str("hello, world");
    s.push('!');

    assert_eq!(s, "hello, world!");

    println!("Success! test_string_03");
}

#[test]
// Fix all errors without adding newline
fn test_string_04() {
    let mut s = String::from("hello");
    s.push(',');
    s.push_str(" world");
    s += "!";

    println!("{}\n Success! test_string_04 ", s);
}

#[test]

// Fill the blank
fn test_string_05() {
    let s = String::from("I like dogs");
    // Allocate new memory and store the modified string there
    let s1 = s.replace("dogs", "cats");

    assert_eq!(s1, "I like cats");

    println!("Success! test_string_05");
}

#[test]

// Fix errors without removing any line
fn test_string_06() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2;
    assert_eq!(s3, "hello,world!");
    println!("{}\n Success! test_string_06", s3);
}

#[test]

// Fix error with at least two solutions
fn test_string_07_01() {
    let s = "hello, world".to_string();
    greetings_string_07_01(s);
    println!("Success! test_string_07_01");
}

fn greetings_string_07_01(s: String) {
    println!("{}", s)
}

#[test]

// Fix error with at least two solutions
fn test_string_07_02() {
    let s = String::from("hello, world");
    greetings_string_07_02(s);
    println!("Success! test_string_07_02");
}

fn greetings_string_07_02(s: String) {
    println!("{}", s)
}

#[test]

// Use two approaches to fix the error and without adding a new line
fn test_string_08_01() {
    let s = "hello, world".to_string();
    let s1: &str = &s;

    println!("Success! test_string_08_01");
}

#[test]

// Use two approaches to fix the error and without adding a new line
fn test_string_08_02() {
    let s = "hello, world";
    let s1: &str = s;

    println!("Success! test_string_08_02");
}
#[test]
fn test_string_08_03() {
    let s = "hello, world".to_string();
    let s1: String = s;

    println!("Success! test_string_08_03");
}

#[test]
fn test_string_09() {
    // You can use escapes to write bytes by their hexadecimal values
    // Fill the blank below to show "I'm writing Rust"
    let byte_escape = "I'm writing Ru\x73\x74!"; // add x74
    println!("What are you doing\x3F (\\x3F means ?) {}", byte_escape);

    // ...Or Unicode code points.
    let unicode_codepoint = "\u{211D}";
    let character_name = "\"DOUBLE-STRUCK CAPITAL R\"";

    println!(
        "Unicode character {} (U+211D) is called {}",
        unicode_codepoint, character_name
    );

    let long_string = "String literals
                        can span multiple lines.
                        The linebreak and indentation here \
                         can be escaped too!";
    println!("{}", long_string);
}

#[test]
/* Fill in the blank and fix the errors */
fn test_string_10() {
    let raw_str = "Escapes don't work here: \x3F \u{211D}"; //remove r
                                                            // Modify above line to make it work
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // If you need quotes in a raw string, add a pair of #s
    let quotes = r#"And then I said: "There is no escape!""#;
    println!("{}", quotes);

    // If you need "# in your string, just use more #s in the delimiter.
    // You can use up to 65535 #s.
    let delimiter = r###"A string with "# in it. And even "##!"###;
    println!("{}", delimiter);

    // Fill the blank
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success! test_string_10");
}

#[test]
fn test_string_11() {
    let s1 = String::from("hi,中国");
    let h = &s1[0..1]; // Modify this line to fix the error, tips: `h` only takes 1 byte in UTF8 format
    assert_eq!(h, "h");

    let h1 = &s1[3..6]; // Modify this line to fix the error, tips: `中`  takes 3 bytes in UTF8 format
    assert_eq!(h1, "中");

    println!("Success! test_string_11");
}

#[test]

fn test_string_12() {
    // Fill the blank to print each char in "你好，世界"
    for c in "你好，世界".chars() {
        println!("{}", c)
    }
    println!("Success! test_string_12");
}

//Array

#[test]
fn test_array_01() {
    // Fill the blank with proper array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];

    // Modify the code below to make it work
    assert!(arr.len() == 5);

    println!("Success! test_array_01");
}

#[test]

fn test_array_02() {
    // We can ignore parts of the array type or even the whole type, let the compiler infer it for us
    let arr0 = [1, 2, 3];
    let arr: [_; 3] = ['a', 'b', 'c'];

    // Fill the blank
    // Arrays are stack allocated, `std::mem::size_of_val` returns the bytes which an array occupies
    // A char takes 4 bytes in Rust: Unicode char
    assert!(std::mem::size_of_val(&arr) == 12);

    println!("Success! test_array_02");
}

#[test]
fn test_array_03() {
    // Fill the blank
    let list: [i32; 100] = [1; 100]; // створює масив з одиниць в кількості ста штук

    assert!(list[0] == 1);
    assert!(list.len() == 100);

    println!("Success! test_array_03");
}

#[test]
fn test_array_04() {
    // Fix the error
    let _arr = [1, 2, 3];

    println!("Success! test_array_04");
}

#[test]

fn test_array_05() {
    let arr = ['a', 'b', 'c'];

    let ele = arr[0]; // Only modify this line to make the code work!

    assert!(ele == 'a');

    println!("Success! test_array_05");
}

#[test]

// Fix the error
fn test_array_06() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];

    // `Get` returns an Option<T>, it's safe to use
    let name0 = names.get(0).unwrap();

    // But indexing is not safe
    let _name1 = &names[1];

    println!("Success! test_array_06");
}

//Slice
#[test]

// Fix the errors, DON'T add new lines!
fn test_slice_01() {
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world";

    println!("{:?}\n{}\nSuccess! test_slice_01", s1, s2);
}

#[test]

fn test_slice_02() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..3];

    // Modify '8' to make it work
    // TIPS: slice( reference ) IS NOT an array, if it is an array, then `assert!` will be passed: Each of the two chars '中' and '国'  occupies 4 bytes, 2 * 4 = 8
    assert!(std::mem::size_of_val(&slice) == 16); //Вказівник на елемент на який посилається слайс на 64 бітній архітектурі це 8 байт і довжина слайсу 8 байт

    println!("Success! test_slice_02");
}

#[test]

fn test_slice_03() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    // Fill the blanks to make the code work
    let slice: &[i32] = &arr[1..4];
    assert_eq!(slice, &[2, 3, 4]);

    println!("Success! test_slice_03");
}

#[test]
fn test_slice_04() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // Fill the blank to make the code work, DON'T USE 0..2 again
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);

    println!("Success! test_slice_04");
}

#[test]
fn test_slice_05() {
    let s = "你好，世界";
    // Modify this line to make the code work
    let slice = &s[0..3];

    assert!(slice == "你");

    println!("Success! test_slice_05");
}

#[test]

// Fix errors
fn test_slice_06() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_letter` needs a `&str` type.
    // It works because `&String` can be implicitly converted to `&str. If you want to know more, this is called `Deref coercion`.
    let letter = first_letter_slice_06(&s);

    println!("the first letter is: {}", letter);

    s.clear(); // error!
}
fn first_letter_slice_06(s: &str) -> &str {
    &s[..1]
}

//Tuple
#[test]

fn test_tuple_01() {
    let _t0: (u8, i16) = (0, -1);
    // Tuples can be tuple's members
    let _t1: (u8, (i16, u32)) = (0, (-1, 1));
    // Fill the blanks to make the code work
    let t: (u8, u16, i64, &str, String) = (1u8, 2u16, 3i64, "hello", String::from(", world"));

    println!("Success! test_tuple_01");
}

#[test]
// Make it work
fn test_tuple_02() {
    let t = ("i", "am", "sunface");
    assert_eq!(t.2, "sunface");

    println!("Success! test_tuple_02");
}

#[test]

// Fix the error
fn test_tuple_03() {
    let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12);
    println!("too long tuple: {:?}", too_long_tuple);
}

#[test]

fn test_tuple_04() {
    let tup = (1, 6.4, "hello");

    // Fill the blank to make the code work
    let (x, z, y) = tup;

    assert_eq!(x, 1);
    assert_eq!(y, "hello");
    assert_eq!(z, 6.4);

    println!("Success! test_tuple_04");
}

#[test]
fn test_tuple_05() {
    let (x, y, z);

    // Fill the blank
    (y, z, x) = (1, 2, 3);

    assert_eq!(x, 3);
    assert_eq!(y, 1);
    assert_eq!(z, 2);

    println!("Success! test_tuple_05");
}

#[test]

fn test_tuple_06() {
    // Fill the blank, need a few computations here.
    let (x, y) = sum_multiply_tuple_06((2, 3));

    assert_eq!(x, 5);
    assert_eq!(y, 6);

    println!("Success! test_tuple_06");
}

fn sum_multiply_tuple_06(nums: (i32, i32)) -> (i32, i32) {
    (nums.0 + nums.1, nums.0 * nums.1)
}

//Struct

// Fix the error
struct Person_struct_01 {
    name: String,
    age: u8,
    hobby: String,
}
#[test]
fn test_struct_01() {
    let age = 30;
    let p = Person_struct_01 {
        name: String::from("sunface"),
        age,
        hobby: String::from("golf"),
    };

    println!("Success! test_struct_01");
}

struct Unit_struct_02;
trait SomeTrait_struct_02 {
    // ...Some behavours defines here
}

// We don't care the the fields are in Unit, but we care its behaviors.
// So we use a struct with no fields and implement some behaviors for it
impl SomeTrait_struct_02 for Unit_struct_02 {}
#[test]
fn test_struct_02() {
    let u = Unit_struct_02;
    do_something_with_unit(u);
    println!("Success! test_struct_02");
}

// fill the blank to make the code work
fn do_something_with_unit(u: Unit_struct_02) {}

// Fix the error and fill the blanks
// struct Color(i32, i32, i32);
struct Point_struct_03(i32, i32, i32);
#[test]
fn test_struct_03() {
    let v = Point_struct_03(0, 127, 255);
    check_color_struct_03(v);

    println!("Success! test_struct_03");
}

fn check_color_struct_03(p: Point_struct_03) {
    let Point_struct_03(x, _, _) = p;
    assert_eq!(x, 0);
    assert_eq!(p.1, 127);
    assert_eq!(p.2, 255);
}

// Fill the blank and fix the error without adding/removing new line
struct Person_struct_04 {
    name: String,
    age: u8,
}
#[test]
fn test_struct_04() {
    let age = 18;
    let mut p = Person_struct_04 {
        name: String::from("sunface"),
        age,
    };

    // How can you believe sunface is only 18?
    p.age = 30;

    // Fill the blank
    p.name = String::from("sunfei");

    println!("Success! test_struct_04");
}

// Fill the blank
struct Person_struct_05 {
    name: String,
    age: u8,
}
#[test]
fn test_struct_05() {
    println!("Success! test_struct_05");
}

fn build_person_struct_05(name: String, age: u8) -> Person_struct_05 {
    Person_struct_05 { age, name }
}

// Fill the blank to make the code work
struct User_struct_06 {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
#[test]
fn test_struct_06() {
    let u1 = User_struct_06 {
        email: String::from("someone@example.com"),
        username: String::from("sunface"),
        active: true,
        sign_in_count: 1,
    };

    let u2 = set_email_struct_06(u1);

    println!("Success! test_struct_06");
}

fn set_email_struct_06(u: User_struct_06) -> User_struct_06 {
    User_struct_06 {
        email: String::from("contact@im.dev"),
        ..u
    } //Приймає всі поля з (u), крім явновказанного (email)
}

//Enum

// Fix the errors
enum Number_enum_01 {
    Zero,
    One,
    Two,
}

enum Number1_enum_01 {
    Zero = 0,
    One,
    Two,
}

// C-like enum
enum Number2_enum_01 {
    Zero = 0,
    One = 1,
    Two = 2,
}

#[test]
fn test_enum_01() {
    // An enum variant can be converted to a integer by `as`
    assert_eq!(Number_enum_01::One as u8, Number_enum_01::One as u8);
    assert_eq!(Number1_enum_01::One as u8, Number2_enum_01::One as u8);

    println!("Success! test_enum_01");
}

// Fill in the blank
enum Message_enum_02 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test_enum_02() {
    let msg1 = Message_enum_02::Move { x: 1, y: 2 }; // Instantiating with x = 1, y = 2
    let msg2 = Message_enum_02::Write(String::from("hello, world")); // Instantiating with "hello, world!"

    println!("Success! test_enum_02");
}

// Fill in the blank and fix the error
enum Message_enum_03 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test_enum_03() {
    let msg = Message_enum_03::Move { x: 1, y: 1 };

    if let Message_enum_03::Move { x: a, y: b } = msg {
        assert_eq!(a, b);
    } else {
        panic!("NEVER LET THIS RUN！");
    }

    println!("Success! test_enum_03");
}

#[test]
// Fill in the blank to make the `println` work.
// Also add some code to prevent the `panic` from running.
fn test_enum_05() {
    let five = Some(5);
    let six = plus_one_enum_05(five);
    let none = plus_one_enum_05(None);

    if let Some(n) = six {
        println!("{}", n);

        println!("Success! test_enum_05");
    } else {
        panic!("NEVER LET THIS RUN！");
    }
}

fn plus_one_enum_05(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
