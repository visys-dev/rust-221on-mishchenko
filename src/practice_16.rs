//Практична робота 16
use itertools::Itertools;
use std::collections::HashMap;

fn solve_crypto_puzzle(equation: &str) -> (Vec<HashMap<char, usize>>, usize) {
    let parts: Vec<&str> = equation.split(" = ").collect();
    let left_parts: Vec<&str> = parts[0].split(" * ").collect();


    let mut letters = parts[0]
        .chars()
        .chain(parts[1].chars())
        .unique()
        .collect::<Vec<_>>();
    letters.sort();

    let mut solutions = Vec::new();


    for nums in (1..=8).into_iter().permutations(letters.len()) {
        let mapping: HashMap<char, usize> = letters
            .iter()
            .zip(nums.iter())
            .map(|(c, n)| (*c, *n))
            .collect();


        let left_value: usize = left_parts[0]
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| mapping[&c] * 10_usize.pow(i as u32))
            .sum();

        let right_value: usize = parts[1]
            .chars()
            .rev()
            .enumerate()
            .map(|(i, c)| mapping[&c] * 10_usize.pow(i as u32))
            .sum();


        if left_value
            + mapping[&left_parts[1].chars().next().unwrap()]
                * mapping[&left_parts[1].chars().nth(1).unwrap()]
            == right_value
        {
            solutions.push(mapping);
        }
    }

    (solutions.clone(), solutions.len())
}
#[test]
fn pr16_main() {
    let equation = "Muxa * a = Slon";
    let (solutions, num_solutions) = solve_crypto_puzzle(equation);

    for solution in &solutions {
        print!("  ");
        for c in &['m', 'u', 'x', 'a'] {
            print!("{}", solution[c]);
        }
        print!(" x ");
        print!("{} ------ ", solution[&'a']);
        for c in &['s', 'l', 'o', 'n'] {
            print!("{}", solution[c]);
        }
        println!();
    }

    println!("Кількість рішень: {}", num_solutions);
}
