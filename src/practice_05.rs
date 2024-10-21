// Практична робота №5 (Малювання конверта)
#[test]
fn test_01() {
    const W: u32 = 30;
    const H: u32 = 15;

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let is_main_diagonal = x == y * (W - 1) / (H - 1);
            let is_secondary_diagonal = x == (H - 1 - y) * (W - 1) / (H - 1);

            let c = if is_horizontal || is_vertical || is_main_diagonal || is_secondary_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}