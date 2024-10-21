// Практична робота №12 (Random vector)
use rand::Rng;

fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..=99)).collect()
}

fn min_adjacent_sum(data: &[i32]) -> Option<(i32, i32, usize, usize, i32)> {
    if data.len() < 2 {
        return None;
    }

    let mut min_sum_pair = (data[0], data[1]);
    let mut min_sum = data[0] + data[1];
    let mut min_indices = (0, 1);

    for i in 1..data.len() - 1 {
        let current_sum = data[i] + data[i + 1];
        if current_sum < min_sum {
            min_sum = current_sum;
            min_sum_pair = (data[i], data[i + 1]);
            min_indices = (i, i + 1);
        }
    }

    Some((
        min_sum_pair.0,
        min_sum_pair.1,
        min_indices.0,
        min_indices.1,
        min_sum,
    ))
}

#[test]
fn test_practice_12() {
    let random_vector = gen_random_vector(20);
    println!("Згенерований рандомний вектор: {:?}", random_vector);

    if let Some((a, b, idx1, idx2, sum)) = min_adjacent_sum(&random_vector) {
        println!(
            "Мінімальна сума пари: {} + {} = {} на індексах: {}, {}",
            a, b, sum, idx1, idx2
        );
    } else {
        println!("Дані не містять достатньо елементів для обчислення пари.");
    }
}
