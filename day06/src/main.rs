#[derive(Clone)]
struct Input {
    tiles: Vec<u8>,
    width: usize,
    height: usize,
}

fn main() {
    let input = read_input("input.txt");
    println!("The solution ot part 1 is {}", part1(input));
    let input = read_input("input.txt");
    println!("The solution ot part 2 is {}", part2(input));
}

fn iter_grid(width: usize, height: usize) -> impl Iterator<Item = (i32, i32)> {
    (0..width)
        .map(move |x| (0..height).map(move |y| (x as i32, y as i32)))
        .flatten()
}

fn part1(mut input: Input) -> usize {
    let final_guard_position = loop {
        let (guard_location, guard_tile) = iter_grid(input.width, input.height)
            .map(|pos| (pos, get(&input, pos).unwrap()).clone())
            .find(|(_, &tile)| is_guard(tile))
            .unwrap();
        let guard_tile = guard_tile.clone();
        let dir = match guard_tile.clone() {
            b'^' => (0, -1),
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            _ => panic!(""),
        };
        let next_tile_location = (guard_location.0 + dir.0, guard_location.1 + dir.1);
        let next_tile = match get(&input, next_tile_location) {
            Some(&next_tile) => next_tile,
            None => break guard_location, // guard moved outside the field
        };

        match next_tile {
            b'X' | b'.' => {
                *get_mut(&mut input, guard_location).unwrap() = b'X';
                *get_mut(&mut input, next_tile_location).unwrap() = guard_tile;
            }
            b'#' => {
                *get_mut(&mut input, guard_location).unwrap() = rotate(guard_tile);
            }
            _ => panic!(),
        }
    };
    *get_mut(&mut input, final_guard_position).unwrap() = b'X';

    for j in 0..input.height {
        for i in 0..input.width {
            print!(
                "{}",
                get(&input, (i as i32, j as i32)).cloned().unwrap() as char
            );
        }
        print!("\n");
    }

    input.tiles.iter().filter(|&&c| c == b'X').count()
}

fn part2(input: Input) -> usize {
    iter_grid(input.width, input.height)
        .filter(|&pos| get(&input, pos).unwrap().clone() == b'.')
        .filter(|&pos| {
            let mut new_input = input.clone();
            *get_mut(&mut new_input, pos).unwrap() = b'#';
            loops(new_input)
        })
        .count()
}

fn loops(mut input: Input) -> bool {
    #[derive(Clone, Copy)]
    struct VistationRecord {
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    }
    let mut records: Vec<VistationRecord> = iter_grid(input.width, input.height)
        .map(|_| VistationRecord {
            up: false,
            down: false,
            left: false,
            right: false,
        })
        .collect();

    let (mut guard_position, _) = iter_grid(input.width, input.height)
        .map(|pos| (pos, get(&input, pos).unwrap().clone()))
        .find(|(_, tile)| is_guard(*tile))
        .unwrap();

    let (final_guard_position, looped) = loop {
        let guard_tile = get(&input, guard_position).unwrap().clone();
        let dir = match guard_tile.clone() {
            b'^' => (0, -1),
            b'>' => (1, 0),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            _ => panic!(""),
        };
        let r = records[(guard_position.0 + guard_position.1 * input.width as i32) as usize];
        match guard_tile.clone() {
            b'^' => {
                if r.up {
                    break (guard_position, true);
                }
            }
            b'v' => {
                if r.down {
                    break (guard_position, true);
                }
            }
            b'<' => {
                if r.left {
                    break (guard_position, true);
                }
            }
            b'>' => {
                if r.right {
                    break (guard_position, true);
                }
            }
            _ => {}
        };
        records[(guard_position.0 + guard_position.1 * input.width as i32) as usize] =
            match guard_tile {
                b'^' => VistationRecord { up: true, ..r },
                b'>' => VistationRecord { right: true, ..r },
                b'v' => VistationRecord { down: true, ..r },
                b'<' => VistationRecord { left: true, ..r },
                _ => panic!(),
            };
        let next_tile_location = (guard_position.0 + dir.0, guard_position.1 + dir.1);
        let next_tile = match get(&input, next_tile_location) {
            Some(&next_tile) => next_tile,
            None => break (guard_position, false), // guard moved outside the field
        };

        match next_tile {
            b'X' | b'.' => {
                *get_mut(&mut input, guard_position).unwrap() = b'X';
                *get_mut(&mut input, next_tile_location).unwrap() = guard_tile;
                guard_position = next_tile_location;
            }

            b'#' => {
                *get_mut(&mut input, guard_position).unwrap() = rotate(guard_tile);
            }

            _ => panic!(),
        }

    };

    *get_mut(&mut input, final_guard_position).unwrap() = b'X';
    if looped {
        true
    } else {
        false
    }
}


fn is_guard(c: u8) -> bool {
    match c {
        b'^' => true,
        b'>' => true,
        b'v' => true,
        b'<' => true,
        _ => false,
    }
}

fn rotate(guard: u8) -> u8 {
    match guard {
        b'^' => b'>',
        b'>' => b'v',
        b'v' => b'<',
        b'<' => b'^',
        _ => panic!(),
    }
}

fn get_mut(input: &mut Input, (x, y): (i32, i32)) -> Option<&mut u8> {
    let idx = get_get_idx(input.width, input.height, x, y)?;
    Some(&mut input.tiles[idx as usize])
}

fn get(input: &Input, (x, y): (i32, i32)) -> Option<&u8> {
    let idx = get_get_idx(input.width, input.height, x, y)?;
    Some(&input.tiles[idx as usize])
}

fn get_get_idx(width: usize, height: usize, x: i32, y: i32) -> Option<usize> {
    let width = width as i32;
    let height = height as i32;
    if x < 0 || x >= width || y < 0 || y >= height {
        return None;
    }
    Some((x + y * width) as usize)
}

fn read_input(path: &str) -> Input {
    let file_content = std::fs::read_to_string(path).expect("could not read file");

    let width = file_content.lines().next().expect("input is empty").len();
    for line in file_content.lines() {
        assert_eq!(line.len(), width)
    }

    let mut tiles = Vec::<u8>::new();
    let mut height = 0;
    for line in file_content.lines() {
        for b in line.bytes() {
            tiles.push(b);
        }
        height += 1;
    }

    Input {
        tiles,
        width,
        height,
    }
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(input), 41);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(input), 6);
}
