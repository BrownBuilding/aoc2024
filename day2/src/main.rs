fn main() {
    let input = read_input("input.txt");
    println!("Solution to part 1: {}", input.iter().filter(|&report|is_safe(&report)).count());
    println!("Solution to part 2: {}", input.iter().filter(|&report|is_safe_with_tolerance(&report)).count());
}

fn read_input(path: &str) -> Vec<Vec<i32>> {
    let input = std::fs::read_to_string(path).expect("could not read input file");
    input.lines()
        .map(|line| line
            .split_whitespace()
            .map(|nstr| nstr
                .parse()
                .expect("could not parse number")
            )
            .collect()
        ).collect::<Vec<Vec<i32>>>()
}

fn is_safe(report: &[i32]) -> bool {
    is_safely_increaseing(report) || is_safely_decreaseing(report)
}

fn is_safe_with_tolerance(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut report_vec = report.to_vec();
        report_vec.remove(i);
        if is_safe(&report_vec) {
            return true;
        }
    }
    false
}

fn is_safely_increaseing(report: &[i32]) -> bool {
    let mut it = report.iter().peekable();
    while let (Some(&a), Some(&&b)) = (it.next(), it.peek()) {
        let d = a - b; // d must be negative
        if d < -3 || d >= 0 {
            return false;
        }
    }
    true
}

fn is_safely_decreaseing(report: &[i32]) -> bool {
    let mut it = report.iter().peekable();
    while let (Some(&a), Some(&&b)) = (it.next(), it.peek()) {
        let d = a - b; // d must be positive
        if d > 3 || d <= 0 {
            return false;
        }
    }
    true
}

#[test] 
fn test_part1() {
    let input = read_input("test_input.txt");
    let result = input.iter().filter(|&report|is_safe(&report)).count();
    assert_eq!(result, 2);
}

#[test] 
fn test_part2() {
    let input = read_input("test_input.txt");
    let result = input.iter().filter(|&report|is_safe_with_tolerance(&report)).count();
    assert_eq!(result, 4);
}
