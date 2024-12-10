fn read_input(path: &str) -> Vec<Vec<u32>> {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("not a digit"))
                .collect()
        })
        .collect()
}

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

fn part2(input: &Vec<Vec<u32>>) -> usize {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let score = trail_rating(input, (row as i32, col as i32), None);
            sum += score;
        }
    }
    sum
}

fn part1(input: &Vec<Vec<u32>>) -> usize {
    let mut sum = 0;
    for row in 0..input.len() {
        for col in 0..input[row].len() {
            let score = number_of_reachable_tops(input, (row as i32, col as i32), None, &mut Vec::new());
            sum += score;
        }
    }
    sum
}

fn trail_rating(input: &Vec<Vec<u32>>, pos: (i32, i32), last_value: Option<u32>) -> usize {
    let value = match get(input, pos) {
        Some(value) => value,
        None => return 0,
    };
    if let Some(last_value) = last_value {
        if value as i32 - last_value as i32 != 1 {
            return 0;
        }
    } else {
        if value != 0 {
            return 0;
        }
    }
    if value == 9 {
        // if found_tops.contains(&pos) {
        //     return 0;
        // };
        // found_tops.push(pos);
        return 1;
    }
    let (row, col) = pos;
    trail_rating(input, (row + 1, col), Some(value))
        + trail_rating(input, (row, col + 1), Some(value))
        + trail_rating(input, (row - 1, col), Some(value))
        + trail_rating(input, (row, col - 1), Some(value))
}

fn number_of_reachable_tops(input: &Vec<Vec<u32>>, pos: (i32, i32), last_value: Option<u32>, found_tops: &mut Vec<(i32, i32)>) -> usize {
    let value = match get(input, pos) {
        Some(value) => value,
        None => return 0,
    };
    if let Some(last_value) = last_value {
        if value as i32 - last_value as i32 != 1 {
            return 0;
        }
    } else {
        if value != 0 {
            return 0;
        }
    }
    if value == 9 {
        if found_tops.contains(&pos) {
            return 0;
        };
        found_tops.push(pos);
        return 1;
    }
    let (row, col) = pos;
    number_of_reachable_tops(input, (row + 1, col), Some(value), found_tops)
        + number_of_reachable_tops(input, (row, col + 1), Some(value), found_tops)
        + number_of_reachable_tops(input, (row - 1, col), Some(value), found_tops)
        + number_of_reachable_tops(input, (row, col - 1), Some(value), found_tops)
}

fn get(input: &Vec<Vec<u32>>, pos: (i32, i32)) -> Option<u32> {
    if pos.0 >= input.len() as i32 || pos.0 < 0 {
        return None;
    }
    if pos.1 >= input[pos.0 as usize].len() as i32 || pos.1 < 0 {
        return None;
    }
    Some(input[pos.0 as usize][pos.1 as usize])
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(&input), 36);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(&input), 81);
}
