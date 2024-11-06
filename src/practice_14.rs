//Практична робота №14 (Rectangles)
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    a: Point,
    b: Point,
}

impl Rectangle {
    fn area(&self) -> i32 {
        (self.b.x - self.a.x).abs() * (self.b.y - self.a.y).abs()
    }

    fn intersection(&self, other: &Rectangle) -> Option<Rectangle> {
        let x1 = self.a.x.max(other.a.x);
        let y1 = self.a.y.min(other.a.y);
        let x2 = self.b.x.min(other.b.x);
        let y2 = self.b.y.max(other.b.y);

        if x1 < x2 && y1 > y2 {
            Some(Rectangle {
                a: Point { x: x1, y: y1 },
                b: Point { x: x2, y: y2 },
            })
        } else {
            None
        }
    }
}

fn area_occupied(xs: &Vec<Rectangle>) -> i32 {
    let mut total_area = 0;

    for i in 0..xs.len() {
        total_area += xs[i].area();
    }

    for i in 0..xs.len() {
        for j in i + 1..xs.len() {
            if let Some(intersection) = xs[i].intersection(&xs[j]) {
                total_area -= intersection.area();
            }
        }
    }

    total_area
}

fn test_data() -> Vec<Rectangle> {
    vec![
        Rectangle {
            a: Point { x: 2, y: 9 },
            b: Point { x: 5, y: 3 },
        },
        Rectangle {
            a: Point { x: 1, y: 8 },
            b: Point { x: 11, y: 6 },
        },
        Rectangle {
            a: Point { x: 9, y: 10 },
            b: Point { x: 13, y: 2 },
        },
    ]
}

fn area_occupied_test() {
    let data = test_data();
    let occupied = area_occupied(&data);
    assert_eq!(occupied, 60);
}

#[test]
fn pr14_test() {
    area_occupied_test();
}

