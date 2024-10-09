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
