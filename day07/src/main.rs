#[derive(Debug)]
struct Line {
    result: i128,
    terms: Vec<i128>,
}

fn read_input(path: &str) -> Vec<Line> {
    let file_content = std::fs::read_to_string(path).expect("could not find input file");
    let strline_to_line = |line: &str| -> Line {
        let (result_str, terms_str) = line.split_once(": ").expect("invalid format");
        let result: i128 = result_str.parse().unwrap();
        let terms: Vec<i128> = terms_str
            .split(' ')
            .map(|term_str| term_str.parse().unwrap())
            .collect();
        Line { result, terms }
    };
    file_content.lines().map(strline_to_line).collect()
}

fn part1(input: Vec<Line>) -> i128 {
    input
        .iter()
        .filter(|line| solvable_part1(line.result, 0, &line.terms))
        .map(|line| line.result)
        .sum()
}

fn solvable_part1(expected_result: i128, tentative_result: i128, remaining_terms: &[i128]) -> bool {
    match remaining_terms {
        [] => expected_result == tentative_result,
        [head, tail @ ..] => {
            if solvable_part1(expected_result, tentative_result + head, tail) {
                return true;
            }
            if solvable_part1(expected_result, tentative_result * head, tail) {
                return true;
            }
            false
        }
    }
}

fn part2(input: Vec<Line>,) -> i128 {
    input
        .iter()
        .filter(|line| solvable_part2(line.result, 0, &line.terms))
        .map(|line| line.result)
        .sum()
}

fn solvable_part2(expected_result: i128, tentative_result: i128, remaining_terms: &[i128]) -> bool {
    // i guess it can give a false positive if the result can be obtained by concatinating a zero
    // for the first operation which is possible because the first tentative_result will always be
    // zero. I don't think that is permitted in the original task.
    match remaining_terms {
        [] => expected_result == tentative_result,
        [head, tail @ ..] => {
            if solvable_part2(expected_result, tentative_result + head, tail) {
                return true;
            }
            if solvable_part2(expected_result, tentative_result * head, tail) {
                return true;
            }
            if solvable_part2(
                expected_result,
                (tentative_result.to_string() + &head.to_string())
                    .parse()
                    .unwrap(),
                tail,
            ) {
                return true;
            }
            false
        }
    }
}

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(input));
    let input = read_input("input.txt");
    println!("The solution to part 2 is {}", part2(input));
}

#[test]
fn test_part1() {
    let test_input = read_input("testinput.txt");
    assert_eq!(part1(test_input), 3749)
}

#[test]
fn test_part2() {
    let test_input = read_input("testinput.txt");
    assert_eq!(part2(test_input), 11387)
}
