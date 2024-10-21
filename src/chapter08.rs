//Match, if let

// Fill the blanks
enum Direction_test_01 {
    East,
    West,
    North,
    South,
}
#[test]
fn test_match_01() {
    let dire = Direction_test_01::South;
    match dire {
        Direction_test_01::East => println!("East"),
        Direction_test_01::South | Direction_test_01::North => {
            // Matching South or North here
            println!("South or North");
        }
        _ => println!("West"),
    };
}

#[test]
fn test_match_02() {
    let boolean = true;

    // Fill the blank with a match expression:
    //
    // boolean = true => binary = 1
    // boolean = false =>  binary = 0
    let binary = match boolean {
        true => 1,
        false => 0,
    };

    assert_eq!(binary, 1);

    println!("Success!");
}

// Fill in the blanks
enum Message_match_03 {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
#[test]
fn test_match_03() {
    let msgs = [
        Message_match_03::Quit,
        Message_match_03::Move { x: 1, y: 3 },
        Message_match_03::ChangeColor(255, 255, 0),
    ];

    for msg in msgs {
        show_message_match_03(msg)
    }

    println!("Success!");
}

fn show_message_match_03(msg: Message_match_03) {
    match msg {
        Message_match_03::Move { x: a, y: b } => {
            // match  Message::Move
            assert_eq!(a, 1);
            assert_eq!(b, 3);
        }
        Message_match_03::ChangeColor(_, g, b) => {
            assert_eq!(g, 255);
            assert_eq!(b, 0);
        }
        __ => println!("no data in these variants"),
    }
}

#[test]
fn test_match_04() {
    let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];

    // Fill the blank with `matches!` to make the code work
    for ab in alphabets {
        assert!(matches!(ab, 'a'..='z' | 'A'..='Z' | '0'..='9'));
    }

    println!("Success!");
}

enum MyEnum_match_05 {
    Foo,
    Bar,
}
#[test]
fn test_match_05() {
    let mut count = 0;

    let v = vec![
        MyEnum_match_05::Foo,
        MyEnum_match_05::Bar,
        MyEnum_match_05::Foo,
    ];
    for e in v {
        if matches!(e, MyEnum_match_05::Foo) {
            // fix the error with changing only this line
            count += 1;
        }
    }

    assert_eq!(count, 2);
}

#[test]
fn test_match_06() {
    let o = Some(7);

    if let Some(i) = o {
        println!("This is a really long string and `{:?}`", i);
    }
}



// Fill in the blank
enum Foo_match_06 {
    Bar(u8)
}
#[test]
fn test_match_07() {
    let a = Foo_match_06::Bar(1);

    if let Foo_match_06::Bar(i) = a {
        println!("foobar holds the value: {}", i);

        println!("Success!");
    }
}


enum Foo_test_08 {
    Bar,
    Baz,
    Qux(u32)
}
#[test]
fn test_match_08() {
    let a = Foo_test_08::Qux(10);

    match a {
        Foo_test_08::Bar => {println!("match Foo_test_08:bar")},
        Foo_test_08::Baz => {println!("match Foo_test_08:baz")},
        _ => println!("match others")
    }
}


#[test]
// Fix the errors in-place
fn test_match_09() {
    let age = Some(30);
    if let Some(age) = age { // Create a new variable with the same name as previous `age`
        assert_eq!(age, 30);
    } // The new variable `age` goes out of scope here

    match age {
        // Match can also introduce a new shadowed variable
        Some(age) =>  println!("age is a new variable, it's value is {}",age),
        _ => ()
    }
}