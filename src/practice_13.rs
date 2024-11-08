use std::io;

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn min_moves_to_balance_cargo(n: i32, weights: &Vec<i32>) -> i32 {
    let total_weight: i32 = weights.iter().sum();

    if total_weight % n != 0 {
        return -1;
    }

    let target_weight = total_weight / n;
    let mut moves = 0;

    for &weight in weights {
        if weight > target_weight {
            moves += weight - target_weight;
        }
    }

    for &weight in weights {
        if (weight - target_weight).abs() > moves {
            return -1;
        }
    }

    moves
}

fn main() {
    let n: i32 = read_line().parse().expect("Invalid input for N");

    let weights: Vec<i32> = read_line()
        .split_whitespace()
        .map(|x| x.parse().expect("Invalid weight"))
        .collect();

    let result = min_moves_to_balance_cargo(n, &weights);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let weights = vec![1, 2, 3, 4, 0];
        assert_eq!(min_moves_to_balance_cargo(5, &weights), 3);
        println!("test_basic_case success!");
    }

    #[test]
    fn test_impossible_case() {
        let weights = vec![1, 2, 3];
        assert_eq!(min_moves_to_balance_cargo(3, &weights), -1);
    }

    #[test]
    fn test_already_balanced() {
        let weights = vec![2, 2, 2, 2];
        assert_eq!(min_moves_to_balance_cargo(4, &weights), 0);
    }

    #[test]
    fn test_edge_cases() {
        let weights = vec![0, 0, 6];
        assert_eq!(min_moves_to_balance_cargo(3, &weights), -1);

        let weights = vec![5];
        assert_eq!(min_moves_to_balance_cargo(1, &weights), 0);
    }
}
