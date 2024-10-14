// Практична робота №7 (Christmas Tree)

fn draw_christmas_tree(triangles: usize) {
    let max_width = 1 + 2 * (triangles + 2);

    for i in 0..triangles {
        let height = i + 3;
        for row in 0..height {
            let stars = 1 + row * 2;
            let spaces = (max_width - stars) / 2;
            println!("{}{}", " ".repeat(spaces), "*".repeat(stars));
        }
    }
}

#[test]
fn test_01() {
    let tree_size = 4;
    draw_christmas_tree(tree_size);
}
