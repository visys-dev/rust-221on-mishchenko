// Практична робота №4 (Малювання ромба)
#[test]
fn test_01() {
    const SIZE: u8 = 15;
    let half = SIZE / 2;

    let mirror = |a: u8| -> u8 { SIZE - 1 - a };

    for y in 0..SIZE {
        for x in 0..SIZE {
            let min_x = x.min(mirror(x));
            let min_y = y.min(mirror(y));

            let condition = min_x + min_y < half;

            let c = if condition { " " } else { "*" };

            print!("{}", c);
        }

        println!();
    }
}
