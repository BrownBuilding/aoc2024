// use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (numbers_left, numbers_right) = {
        let mut list0 = Vec::<i32>::new();
        let mut list1 = Vec::<i32>::new();
        for line in input.lines() {
            let mut wi = line.split_whitespace();
            let n0 = wi
                .next()
                .expect("expected a number")
                .parse()
                .expect("could not read number");
            let n1 = wi
                .next()
                .expect("expected a number")
                .parse()
                .expect("could not read number");
            list0.push(n0);
            list1.push(n1);
        }
        (list0, list1)
    };

    // part 1:
    let list_zip_sorted: Vec<(i32, i32)> = {
        let mut list0_sorted: Vec<i32> = numbers_left.iter().cloned().collect();
        list0_sorted.sort();
        let mut list1_sorted: Vec<i32> = numbers_right.iter().cloned().collect();
        list1_sorted.sort();
        let list: Vec<(i32, i32)> = list0_sorted
            .iter()
            .cloned()
            .zip(list1_sorted.iter().cloned())
            .collect();
        list
    };

    let sum = list_zip_sorted
        .iter()
        .map(|&(n0, n1)| (n0 - n1).abs())
        .sum::<i32>();

    println!("Solution to part 1: {}", sum);

    // part 2:

    let occurrences = |nl| numbers_right.iter().cloned().filter(|&nr| nr == nl).count();

    let sum: i32 = numbers_left
        .iter()
        .map(|n| n * occurrences(*n) as i32)
        .sum();

    println!("Solution to part 2: {}", sum);
}
