struct Input {
    rules: Vec<(i32, i32)>,
    queues: Vec<Vec<i32>>,
}

fn from_file(path: &str) -> Input {
    let content = std::fs::read_to_string(path).expect("could not read file");
    let mut lines = content.lines();
    let mut orderings = Vec::new();
    // parsing orderings
    while let Some(l) = lines.next() {
        if l.is_empty() {
            break;
        }
        let (n0, n1) = l.split_once('|').expect("uhh");
        let n0 = n0.parse().expect("could not uhh");
        let n1 = n1.parse().expect("could not u");
        orderings.push((n0, n1));
    }

    let queues = lines
        .map(|l| l.split(',').map(|n_str| n_str.parse().unwrap()).collect())
        .collect();

    Input {
        rules: orderings,
        queues,
    }
}

fn main() {
    let input = from_file("input.txt");
    let solution = part_1(input);
    println!("The solution to part 1 is {}", solution);
    let input = from_file("input.txt");
    let solution = part_2(input);
    println!("The solution to part 2 is {}", solution);
}

fn part_1(Input { rules, queues }: Input) -> i32 {
    queues
        .iter()
        .filter(|q| is_correct(q, &rules))
        .map(|q| q[q.len() / 2])
        .sum()
}

fn is_correct(queue: &Vec<i32>, rules: &Vec<(i32, i32)>) -> bool {
    for i in 0..queue.len() {
        for j in 0..i {
            let disallowing_rule = (queue[i], queue[j]);
            if rules.contains(&disallowing_rule) {
                return false;
            }
        }
        for j in i..queue.len() {
            let disallowing_rule = (queue[j], queue[i]);
            if rules.contains(&disallowing_rule) {
                return false;
            }
        }
    }
    true
}

fn part_2(Input { rules, queues }: Input) -> i32 {
    queues
        .into_iter()
        .filter(|q| !is_correct(q, &rules))
        .map(|incorrect_queue| correct_queue(incorrect_queue, &rules))
        // .inspect(|correct_queue| assert!(is_correct(&correct_queue, &rules)))
        .map(|corrected_queue| corrected_queue[corrected_queue.len() / 2])
        .inspect(|middle| println!("{}", middle))
        .sum()
}

fn correct_queue(queue: Vec<i32>, rules: &Vec<(i32, i32)>) -> Vec<i32> {
    // it's not my fault they let me solve the this part the stupid way
    let mut test_queue = Vec::new();
    let queue_len = queue.len();
    for e in queue {
        for i in 0..(test_queue.len() + 1) {
            test_queue.insert(i, e);
            if is_correct(&test_queue, &rules) {
                break;
            }
            test_queue.remove(i);
        }
    }
    assert_eq!(test_queue.len(), queue_len);
    test_queue
}

#[test]
fn test_part1() {
    let input = from_file("testinput.txt");
    let solution = part_1(input);
    assert_eq!(solution, 143);
}

#[test]
fn test_part2() {
    let input = from_file("testinput.txt");
    let solution = part_2(input);
    assert_eq!(solution, 123);
}
