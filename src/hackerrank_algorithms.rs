//#1 Solve me first
use std::io;

fn main() {
    // variable declaration
    let mut _num_str_1 = String::new();
    let mut _num_str_2 = String::new();

    // read variables
    io::stdin()
        .read_line(&mut _num_str_1)
        .ok()
        .expect("read error");
    io::stdin()
        .read_line(&mut _num_str_2)
        .ok()
        .expect("read error");

    // parse integers
    let mut _num_1: i32 = _num_str_1.trim().parse().ok().expect("parse error");
    let mut _num_2: i32 = _num_str_2.trim().parse().ok().expect("parse error");

    // print the sum
    let sum = _num_1 + _num_2;
    // Hint: Type println!("{}", _num_1 + _num_2); below
    println!("{}", sum)
}

//#2 Simple Array Sum
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'simpleArraySum' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY ar as parameter.
 */

fn simpleArraySum(ar: &[i32]) -> i32 {
    // ar.iter().sum()
    let mut sum = 0;
    for &num in ar {
        sum += num;
    }
    return sum;
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = simpleArraySum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}

// #3 Compare the Triplets
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'compareTriplets' function below.
 *
 * The function is expected to return an INTEGER_ARRAY.
 * The function accepts following parameters:
 *  1. INTEGER_ARRAY a
 *  2. INTEGER_ARRAY b
 */

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut alice_points = 0;
    let mut bob_points = 0;

    for i in 0..a.len() {
        if a[i] > b[i] {
            alice_points += 1;
        } else if a[i] < b[i] {
            bob_points += 1;
        }
    }

    return vec![alice_points, bob_points];
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let b: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = compareTriplets(&a, &b);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}

// #4 A Very Big Sum
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'aVeryBigSum' function below.
 *
 * The function is expected to return a LONG_INTEGER.
 * The function accepts LONG_INTEGER_ARRAY ar as parameter.
 */

fn aVeryBigSum(ar: &[i64]) -> i64 {
    let mut ar_sum = 0;

    for i in 0..ar.len() {
        ar_sum += ar[i]
    }
    ar_sum
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _ar_count = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let ar: Vec<i64> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i64>().unwrap())
        .collect();

    let result = aVeryBigSum(&ar);

    writeln!(&mut fptr, "{}", result).ok();
}

// #5 Diagonal Difference
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'diagonalDifference' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts 2D_INTEGER_ARRAY arr as parameter.
 */

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal = 0;
    let mut secondary_diagonal = 0;

    for i in 0..n {
        primary_diagonal += arr[i][i];
        secondary_diagonal += arr[i][n - i - 1];
    }

    (primary_diagonal - secondary_diagonal).abs()
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n as usize);

    for i in 0..n as usize {
        arr.push(Vec::with_capacity(n as usize));

        arr[i] = stdin_iterator
            .next()
            .unwrap()
            .unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<i32>().unwrap())
            .collect();
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}

// #6 Plus Minus
use std::io::{self, BufRead};

/*
 * Complete the 'plusMinus' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn plusMinus(arr: &[i32]) {
    let arr_size = arr.len();
    let mut positive_counter = 0;
    let mut negative_counter = 0;
    let mut zero_counter = 0;

    for i in 0..arr_size {
        if arr[i] > 0 {
            positive_counter += 1;
        } else if arr[i] < 0 {
            negative_counter += 1;
        } else {
            zero_counter += 1;
        }
    }

    println!("{:.6}", positive_counter as f64 / arr_size as f64);
    println!("{:.6}", negative_counter as f64 / arr_size as f64);
    println!("{:.6}", zero_counter as f64 / arr_size as f64);
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let arr: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    plusMinus(&arr);
}

// #7 Staircase
use std::io::{self, BufRead};

/*
 * Complete the 'staircase' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

fn staircase(n: i32) {
    for i in 1..=n {
        for y in 0..(n - i) {
            print!(" ");
        }
        for j in 0..i {
            print!("#");
        }
        println!();
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    staircase(n);
}


// #8 Mini-Max Sum
use std::io::{self, BufRead};

/*
 * Complete the 'miniMaxSum' function below.
 *
 * The function accepts INTEGER_ARRAY arr as parameter.
 */

fn miniMaxSum(arr: &[i32]) {
    let mut total_sum: i64 = 0;
    let mut min_value = i32::MAX;
    let mut max_value = i32::MIN;

    for &value in arr{
        let value_i64 = value as i64;
        total_sum += value_i64;

        if value < min_value{
            min_value = value;
        }
        if value > max_value{
            max_value = value;
        }
    }

    let min_sum = total_sum - max_value as i64;
    let max_sum = total_sum - min_value as i64;

    println!("{} {}", min_sum, max_sum)
}



fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    miniMaxSum(&arr);
}


// #9 Birthday Cake Candles
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'birthdayCakeCandles' function below.
 *
 * The function is expected to return an INTEGER.
 * The function accepts INTEGER_ARRAY candles as parameter.
 */

fn birthdayCakeCandles(candles: &[i32]) -> i32 {
    let mut tallest_candle = candles[0];
    let mut tallest_candles_counter = 0;

    for &candle in candles.iter() {
        if candle > tallest_candle {
            tallest_candle = candle;
            tallest_candles_counter = 1;
        } else if candle == tallest_candle {
            tallest_candles_counter += 1;
        }
    }

    tallest_candles_counter
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _candles_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let candles: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = birthdayCakeCandles(&candles);

    writeln!(&mut fptr, "{}", result).ok();
}


// #10 Time Conversion
use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

/*
 * Complete the 'timeConversion' function below.
 *
 * The function is expected to return a STRING.
 * The function accepts STRING s as parameter.
 */

fn timeConversion(s: &str) -> String {
    let am_or_pm = &s[8..];
    let mut hours = s[0..2].parse::<i32>().unwrap();
    let minutes_seconds = &s[2..8];

    if am_or_pm == "AM" && hours == 12{
        hours = 0;
    } else if am_or_pm == "PM" && hours !=12 {
        hours += 12;
    }

    format!("{:02}{}", hours, minutes_seconds)

}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let s = stdin_iterator.next().unwrap().unwrap();

    let result = timeConversion(&s);

    writeln!(&mut fptr, "{}", result).ok();
}

// #11 Apple and Orange
fn countApplesAndOranges(s: i32, t: i32, a: i32, b: i32, apples: &[i32], oranges: &[i32]) {
    let mut apples_on_house = 0;
    let mut oranges_on_house = 0;

    for &distance in apples {
        let position = a + distance;
        if position >= s && position <= t {
            apples_on_house += 1;
        }
    }


    for &distance in oranges {
        let position = b + distance;
        if position >= s && position <= t {
            oranges_on_house += 1;
        }
    }

    println!("{}", apples_on_house);
    println!("{}", oranges_on_house);

}

// #12 Breaking The Records
fn breaking_records(scores: &[i32]) -> Vec<i32> {
    let mut min_score = scores[0];
    let mut max_score = scores[0];
    let mut min_count = 0;
    let mut max_count = 0;

    for &score in scores.iter().skip(1) {
        if score > max_score {
            max_score = score;
            max_count += 1;
        } else if score < min_score {
            min_score = score;
            min_count += 1;
        }
    }

    vec![max_count, min_count]
}


// #13 Subarray Division
fn birthday(s: &[i32], d: i32, m: i32) -> i32 {
    if m as usize > s.len() {
        return 0;
    }

    let mut count = 0;

    for i in 0..=(s.len() - m as usize) {
        let sum: i32 = s[i..i + m as usize].iter().sum();
        
        if sum == d {
            count += 1;
        }
    }

    count

}

// #14 Divisible Sum Pairs
fn divisibleSumPairs(n: i32, k: i32, ar: &[i32]) -> i32 {
    let mut count = 0;
    
    for i in 0..(n - 1) {
        for j in (i + 1)..n {
            if (ar[i as usize] + ar[j as usize]) % k == 0 {
                count += 1;
            }
        }
    }

    count
    
}

// #15 Migratory Birds
fn migratoryBirds(arr: &[i32]) -> i32 {
    let mut types = vec![0; 5];
    
    for &bird in arr {
        types[(bird - 1) as usize] += 1;
    }
    
    let mut max_freq_value = 0;
    let mut freq_id = 0;

    for (i, &count) in types.iter().enumerate() {
        if count > max_freq_value {
            max_freq_value = count;
            freq_id = (i + 1) as i32;
        }
    }

    freq_id
}


// #16 Day Of the Programmer
fn dayOfProgrammer(year: i32) -> String {
    fn is_julian_leap_year(year: i32) -> bool {
        year % 4 == 0
    }

    fn is_gregorian_leap_year(year: i32) -> bool {
        (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
    }

    if year == 1918 {
        return format!("26.09.1918");
    } else if year < 1918 {
        if is_julian_leap_year(year) {
            return format!("12.09.{}", year);
        } else {
            return format!("13.09.{}", year);
        }
    } else {
        if is_gregorian_leap_year(year) {
            return format!("12.09.{}", year);
        } else {
            return format!("13.09.{}", year);
        }
    }
}


// #17 Bill Division
fn bonAppetit(bill: &[i32], k: i32, b: i32) {
    let total_cost: i32 = bill.iter().enumerate()
        .filter(|(index, _)| *index as i32 != k)
        .map(|(_, &price)| price)
        .sum();

    let anna_share = total_cost / 2;

    if b == anna_share {
        println!("Bon Appetit");
    } else {
        println!("{}", b - anna_share);
    }
}


// #18 Sales by Match
fn sockMerchant(n: i32, ar: &[i32]) -> i32 {
    let max_color = ar.iter().copied().max().unwrap_or(0);
    let mut color_count = vec![0; (max_color + 1) as usize];

    for &sock in ar {
        color_count[sock as usize] += 1;
    }

    let mut pairs = 0;
    for &count in color_count.iter() {
        pairs += count / 2;
    }

    pairs
}


// #19 Drawing Book
fn pageCount(n: i32, p: i32) -> i32 {
    let from_front = p / 2;
    
    let from_back = (n / 2) - (p / 2);
    
    if from_front < from_back {
        from_front
    } else {
        from_back
    }
}

// #20 Counting Valleys
fn countingValleys(steps: i32, path: &str) -> i32 {
    let mut n = 0;
    let mut p = 0;

    for c in path.chars() {
        let l = match c {
            'U' => p + 1,
            'D' => p - 1,
            _ => continue,
        };
        if l == 0 && p < 0 {
            n += 1;
        }
        p = l;
    }

    n
}









