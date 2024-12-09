
const WORD: &[char] = &['X', 'M', 'A', 'S'];

fn main() {
    let input = read_grid("input.txt");

    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += parse(&input, (x as i32, y as i32), 0, Vec::new());
        }
    }
    println!("{}", sum);
}

fn read_grid(path: &str) -> Vec<Vec<u8>> {
    let input: Vec<Vec<u8>> = std::fs::read_to_string(path)
        .expect("could not read input fle")
        .lines()
        .map(|s| s.as_bytes().to_vec())
        .collect();
    let width = input[0].len();
    assert!(input.iter().all(|w| w.len() == width));
    input
}

fn parse(input: &Vec<Vec<u8>>, pos: (i32, i32), current: usize, inspected: Vec<(i32, i32)>) -> usize {
    let (x, y) = pos;

    if x < 0 || x >= input[0].len() as i32 || y < 0 || y >= input.len() as i32 {
        return 0;
    }

    if char::from(input[y as usize][x as usize]) != WORD[current] {
        return 0;
    }

    if inspected.contains(&pos) {
        return 0;
    }

    if current == WORD.len() - 1 {
        return 1;
    }

    let canidates = [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        // (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];

    let mut sum = 0;
    for c in canidates {
        let mut ins_clone = inspected.clone();
        ins_clone.push(pos);
        sum += parse(input, c, current + 1, ins_clone);
    }
    sum
}

#[test]
fn test() {
    let input = read_grid("test_input.txt");
    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += parse(&input, (x as i32, y as i32), 0, Vec::new());
        }
    }
    assert_eq!(sum, 18);
}
