use std::collections::HashMap;

#[derive(Debug)]
struct Input {
    patterns: Vec<String>,
    designs: Vec<String>,
}

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 1 is {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    input
        .designs
        .iter()
        .filter(|d| check(d, &input.patterns, &mut HashMap::new()) != 0)
        .count()
}

fn part2(input: &Input) -> usize {
    input
        .designs
        .iter()
        .map(|d| check(d, &input.patterns, &mut HashMap::new()))
        .sum()
}

fn check(design: &str, patterns: &[String], check_cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }
    let design = design.to_string();
    let mut possible_arrangements = 0;
    if let Some(chached_possible) = check_cache.get(&design) {
        return *chached_possible;
    }
    for p in patterns {
        if let Some(rest) = design.strip_prefix(p) {
            possible_arrangements += check(rest, patterns, check_cache);
        }
    }
    check_cache.insert(design, possible_arrangements);
    possible_arrangements
}

fn read_input(path: &str) -> Input {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    let mut line_iter = file_content.lines();
    let first_line = line_iter.next().unwrap();
    let mut patterns: Vec<String> = first_line.split(", ").map(str::to_string).collect();
    let _ = line_iter.next(); // ignore blank line
    let designs: Vec<String> = line_iter.map(str::to_string).collect();
    patterns.sort_unstable_by(|p0, p1| p1.len().cmp(&p0.len()));
    Input { patterns, designs }
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(&input), 6);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(&input), 16);
}
