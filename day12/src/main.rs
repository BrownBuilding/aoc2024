use std::usize;

type Region = Vec<(i32, i32)>;

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

fn read_input(path: &str) -> Vec<Vec<u8>> {
    let content = std::fs::read_to_string(path).expect("could not find input file");
    let mut input = Vec::<Vec<u8>>::new();
    for line in content.lines() {
        input.push(line.as_bytes().to_vec());
    }
    assert_ne!(input.len(), 0, "input must not be empty");
    let width = input[0].len();
    assert!(
        input.iter().all(|line| line.len() == width),
        "all lines must have the same length"
    );
    input
}

fn part1(input: &Vec<Vec<u8>>) -> i32 {
    let mut sum = 0;
    let mut regions = Vec::<Region>::new();
    for i in 0i32..(input.len() as i32) {
        for j in 0i32..(input.len() as i32) {
            if regions.iter().any(|region| region.contains(&(i, j))) {
                continue;
            }
            let mut region = Region::new();
            let record = claim_region(input, i, j, &mut region, None);
            regions.push(region);
            sum += record.area * record.perimeter;
        }
    }
    sum
}

fn part2(input: &Vec<Vec<u8>>) -> i32 {
    let mut sum = 0;
    let mut regions = Vec::<Region>::new();
    for i in 0i32..(input.len() as i32) {
        for j in 0i32..(input.len() as i32) {
            if regions.iter().any(|region| region.contains(&(i, j))) {
                continue;
            }
            let mut region = Region::new();
            let record = claim_region(input, i, j, &mut region, None);
            let edge_count = count_vertices(&region);
            regions.push(region);
            sum += record.area * edge_count as i32;
        }
    }
    sum
}

#[derive(Debug)]
struct FieldRecord {
    area: i32,
    perimeter: i32,
}

impl std::ops::Add for FieldRecord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            perimeter: self.perimeter + rhs.perimeter,
            area: self.area + rhs.area,
        }
    }
}

fn claim_region(
    input: &Vec<Vec<u8>>,
    x: i32,
    y: i32,
    shape: &mut Region,
    last_field_type: Option<u8>,
) -> FieldRecord {
    let last_field_type = match last_field_type {
        Some(lst) => lst,
        None => input[x as usize][y as usize],
    };
    let this_field_type = match get(input, x, y) {
        Some(tft) => tft,
        None => {
            return FieldRecord {
                area: 0,
                perimeter: 1,
            }
        }
    };
    if last_field_type != this_field_type {
        return FieldRecord {
            area: 0,
            perimeter: 1,
        };
    }
    if shape.contains(&(x, y)) {
        return FieldRecord {
            area: 0,
            perimeter: 0,
        };
    }
    shape.push((x, y));
    claim_region(input, x - 1, y, shape, Some(last_field_type))
        + claim_region(input, x + 1, y, shape, Some(last_field_type))
        + claim_region(input, x, y - 1, shape, Some(last_field_type))
        + claim_region(input, x, y + 1, shape, Some(last_field_type))
        + FieldRecord {
            perimeter: 0,
            area: 1,
        }
}

fn count_vertices(region: &Region) -> usize {
    let does_contain = |x: i32, y: i32| region.iter().cloned().any(|(i, j)| i == x && j == y);
    let true_to_1 = |b| match b {
        true => 1,
        false => 0,
    };
    let number_of_vertices_at_tile = |(x, y): (i32, i32)| -> usize {
        let kernel = [
            [
                does_contain(x - 1, y - 1),
                does_contain(x, y - 1),
                does_contain(x + 1, y - 1),
            ],
            [
                does_contain(x - 1, y),
                does_contain(x, y),
                does_contain(x + 1, y),
            ],
            [
                does_contain(x - 1, y + 1),
                does_contain(x, y + 1),
                does_contain(x + 1, y + 1),
            ],
        ];
        true_to_1(!kernel[2][1] && !kernel[1][2])
            + true_to_1(!kernel[0][1] && !kernel[1][2] && kernel[1][1])
            + true_to_1(!kernel[0][1] && !kernel[1][0] && kernel[1][1])
            + true_to_1(!kernel[2][1] && !kernel[1][0] && kernel[1][1])
            + true_to_1(kernel[2][1] && kernel[1][2] && !kernel[2][2] && kernel[1][1])
            + true_to_1(kernel[0][1] && kernel[1][2] && !kernel[0][2] && kernel[1][1])
            + true_to_1(kernel[0][1] && kernel[1][0] && !kernel[0][0] && kernel[1][1])
            + true_to_1(kernel[2][1] && kernel[1][0] && !kernel[2][0] && kernel[1][1])
    };
    region
        .iter()
        .cloned()
        .map(|tile| number_of_vertices_at_tile(tile))
        .sum()
}

fn get(input: &Vec<Vec<u8>>, x: i32, y: i32) -> Option<u8> {
    input
        .get(x as usize)
        .and_then(|line| line.get(y as usize))
        .cloned()
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(&input), 1930);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(&input), 1206);
}

#[test]
fn test_vertex_count() {
    let results = [
        count_vertices(&vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        count_vertices(&vec![(0, -1), (0, 0), (0, 1), (1, 0), (1, 1)]),
        count_vertices(&vec![(0, 0), (1, 0), (0, 1), (0, 2), (1, 2), (0, 3), (0, 4), (1, 4)]),
    ];
    let expected = [4, 6, 12];
    assert_eq!(results, expected);
}
