use std::collections::HashMap;

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

fn read_input(path: &str) -> Vec<i128> {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    file_content
        .split_whitespace()
        .map(|n_str| n_str.parse().expect("could not parse number"))
        .collect()
}

fn part2(input: &Vec<i128>) -> i128 {
    input.iter()
        .map(|&stone| count_stones(75, stone, &mut HashMap::new()))
        .sum()
}

fn part1(input: &Vec<i128>) -> i128 {
    input.iter()
        .map(|&stone| count_stones(25, stone, &mut HashMap::new()))
        .sum()
}

fn count_stones(blink_amount: i32, stone_value: i128, stone_count_cache: &mut HashMap<(i32, i128), i128>) -> i128 {
    if blink_amount == 0 {
        return 1;
    }
    if stone_value == 0 {
        return count_stones(blink_amount - 1, 1, stone_count_cache);
    }
    if let Some(&cached_stone_count) = stone_count_cache.get(&(blink_amount, stone_value)) {
        return cached_stone_count;
    }
    if let Some((left, right)) = spliteable(stone_value) {
        let count = count_stones(blink_amount - 1, left, stone_count_cache) + count_stones(blink_amount - 1, right, stone_count_cache);
        stone_count_cache.insert((blink_amount, stone_value), count);
        return count
    }
    count_stones(blink_amount - 1, stone_value * 2024, stone_count_cache)
}

fn spliteable(stone_value: i128) -> Option<(i128, i128)> {
    let stone_value_str = stone_value.to_string();
    if stone_value_str.len() % 2 == 0 {
        let (left, right) = stone_value_str.split_at(stone_value_str.len() / 2);
        Some((left.parse().unwrap(), right.parse().unwrap()))
    } else {
        None
    }
}

#[test]
fn test_splitalbe() {
    let results = [
        spliteable(130),
        spliteable(24),
        spliteable(6032),
        spliteable(7303),
    ];
    let expected = [
        None,
        Some((2,4)),
        Some((60, 32)),
        Some((73, 3)),
    ];
    assert_eq!(expected, results);
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(&input), 55312);
}

