#[test]
fn test() {
    assert_eq!(2 + 3, 5);
}

#[test]
fn test1() {
    let x: i32 = 5; // Uninitialized but used, ERROR !
    let _y: i32; // Uninitialized but also unused, only a Warning !

    assert_eq!(x, 5);
    println!("Success!");
}


#[test]
// Fill the blanks in the code to make it compile
fn test2() {
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Success!");
}


// Fix the error below with least amount of modification
#[test]
fn test3() {
    let x: i32 = 10;
    let y: i32; // Оголошення змінної поза блоком
    {
        y = 5; // Присвоєння значення в блоці
        println!("Inner scope value of x is {} and value of y is {}", x, y);
    }
    println!("Outer scope value of x is {} and value of y is {}", x, y);
}


// Fix the error with the use of define_x
#[test]
fn test4()
{
    let x = define_x(); // Викликаємо фунцію
    println!("{} world, {}", x, "https://practice.course.rs/variables.html#scope");
}

fn define_x() -> &'static str // Вказуємо, що функція повертає рядок
{
    let x = "Hello";
    x // Повертаємо x
}

#[test]




