#[test]
fn test_pr_05() {
    const W: u32 = 22;
    const H: u32 = 12;

    for y in 0..H {
        for x in 0..W {
            let is_horizontal = y == 0 || y == H - 1;
            let is_vertical = x == 0 || x == W - 1;
            let is_first_diagonal = x == (y * W) / H;
            let is_second_diagonal = x == (H - 1 - y) * W / H;

            let c = if is_horizontal
                || is_vertical
                || is_first_diagonal
                || is_second_diagonal {
                '*'
            } else {
                ' '
            };
            print!("{}", c);
        }
        println!();
    }
}
