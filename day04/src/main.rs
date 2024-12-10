const WORD_XMAS: &[char] = &['X', 'M', 'A', 'S'];
const WORD_MAS: &[char] = &['M', 'A', 'S'];

fn main() {
    let input = read_grid("input.txt");

    // Part 1
    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += parse_xmas(&input, x as i32, y as i32);
        }
    }
    println!("The solution to part 1 is {}", sum);

    // Part 2
    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += if parse_x_mas(&input, x as i32, y as i32) {
                1
            } else {
                0
            };
        }
    }
    println!("The solution to part 2 is {}", sum);
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

fn parse_xmas(input: &Vec<Vec<u8>>, x: i32, y: i32) -> usize {
    let directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    directions
        .iter()
        .filter(|can| parse_direction(input, x, y, can.0, can.1, WORD_XMAS))
        .count()
}

fn parse_direction(
    input: &Vec<Vec<u8>>,
    mut x: i32,
    mut y: i32,
    dx: i32,
    dy: i32,
    word: &[char],
) -> bool {
    for &c in word {
        if x < 0 || x >= input[0].len() as i32 || y < 0 || y >= input.len() as i32 {
            return false;
        }
        if c != char::from(input[y as usize][x as usize]) {
            return false;
        }
        x += dx;
        y += dy;
    }
    true
}

fn parse_x_mas(input: &Vec<Vec<u8>>, x: i32, y: i32) -> bool {
    if !(parse_direction(input, x, y, 1, 1, WORD_MAS)
        || parse_direction(input, x + 2, y + 2, -1, -1, WORD_MAS))
    {
        return false;
    }
    if !(parse_direction(input, x + 2, y, -1, 1, WORD_MAS)
        || parse_direction(input, x, y + 2, 1, -1, WORD_MAS))
    {
        return false;
    }
    true
}

#[test]
fn test_xmas() {
    let input = read_grid("test_input.txt");
    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += parse_xmas(&input, x as i32, y as i32);
        }
    }
    assert_eq!(sum, 18);
}

#[test]
fn test_x_mas() {
    let input = read_grid("test_input.txt");
    let mut sum = 0;
    for x in 0..input[0].len() {
        for y in 0..input.len() {
            sum += if parse_x_mas(&input, x as i32, y as i32) {
                1
            } else {
                0
            };
        }
    }
    assert_eq!(sum, 9);
}
