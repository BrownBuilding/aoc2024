#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Tile {
    Robot,
    Wall,
    Box,
    Empty,
}

#[derive(Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part2(tiles: &mut Vec<Vec<Tile>>, directions: &[Direction]) -> usize {
    do_steps(tiles, directions);
    let mut sum = 0;
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            if tiles[i][j] == Tile::Box {
                sum += 100 * i + j
            }
        }
    }
    sum
}

fn do_steps(tiles: &mut Vec<Vec<Tile>>, directions: &[Direction]) {
    let mut robot_position = (|| {
        for i in 0..tiles.len() {
            for j in 0..tiles[i].len() {
                match tiles[i][j] {
                    Tile::Robot => return (j as i32, i as i32),
                    Tile::Wall => {}
                    Tile::Box => {}
                    Tile::Empty => {}
                }
            }
        }
        panic!("no robot :(")
    })();

    for &dir in directions {
        // println!();
        // println!(
        //     "current move: {}",
        //     match dir {
        //         Direction::Up => '^',
        //         Direction::Down => 'v',
        //         Direction::Left => '<',
        //         Direction::Right => '>',
        //     }
        // );
        let v = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        if move_box(tiles, add(robot_position, v), dir) {
            *get_mut(tiles, robot_position).unwrap() = Tile::Empty;
            robot_position = add(v, robot_position);
            assert_eq!(get(tiles, robot_position), Some(Tile::Empty));
            *get_mut(tiles, robot_position).unwrap() = Tile::Robot;
        }
    }
}

/// Check if position is part of a box. If so, return the position of the left half of the box.
fn is_box(tiles: &Vec<Vec<Tile>>, pos: (i32, i32)) -> Option<(i32, i32)> {
    if let (Some(Tile::Box), Some(Tile::Empty)) = (get(tiles, pos), get(tiles, add(pos, (1, 0)))) {
        return Some(pos);
    } else if let (Some(Tile::Box), Some(Tile::Empty)) =
        (get(tiles, add(pos, (-1, 0))), get(tiles, pos))
    {
        return Some(add(pos, (-1, 0)));
    };
    None
}

/// Make sure the robot can move into `box_pos`. Move boxes if necessary. Return false if the robot
/// cannot move into `box_pos`.
fn move_box(tiles: &mut Vec<Vec<Tile>>, box_pos: (i32, i32), dir: Direction) -> bool {
    let box_count_before_mutation = tiles
        .iter()
        .map(|line| line.iter())
        .flatten()
        .filter(|&&tile| tile == Tile::Box)
        .count();

    let box_pos = match get(tiles, box_pos) {
        Some(Tile::Box) => box_pos,
        Some(Tile::Empty) => match is_box(tiles, box_pos) {
            Some(box_pos) => box_pos,
            None => return true,
        },
        _ => return false, // there is a wall or the robot wants to move out of bounds
    };

    let mut boxes_that_have_to_be_moved = Vec::new();
    let mut candidates = Vec::new();
    candidates.push(box_pos);
    loop {
        let bo = match candidates.pop() {
            Some(pos) => pos,
            None => break,
        };
        boxes_that_have_to_be_moved.push(bo);
        match dir {
            Direction::Up => {
                if let Some(b) = is_box(tiles, add(bo, (0, -1))) {
                    candidates.push(b);
                }
                if let Some(b) = is_box(tiles, add(bo, (1, -1))) {
                    candidates.push(b);
                }
            }
            Direction::Down => {
                if let Some(b) = is_box(tiles, add(bo, (0, 1))) {
                    candidates.push(b);
                }
                if let Some(b) = is_box(tiles, add(bo, (1, 1))) {
                    candidates.push(b);
                }
            }
            Direction::Left => {
                if let Some(b) = is_box(tiles, add(bo, (-1, 0))) {
                    candidates.push(b);
                }
            }
            Direction::Right => {
                if let Some(b) = is_box(tiles, add(bo, (2, 0))) {
                    candidates.push(b);
                }
            }
        }
    }
    let boxes_cannot_be_moved = boxes_that_have_to_be_moved.iter().any(|&boz| match dir {
        Direction::Up => {
            get(tiles, add(boz, (0, -1))) == Some(Tile::Wall)
                || get(tiles, add(boz, (1, -1))) == Some(Tile::Wall)
        }
        Direction::Down => {
            get(tiles, add(boz, (0, 1))) == Some(Tile::Wall)
                || get(tiles, add(boz, (1, 1))) == Some(Tile::Wall)
        }
        Direction::Left => get(tiles, add(boz, (-1, 0))) == Some(Tile::Wall),
        Direction::Right => get(tiles, add(boz, (2, 0))) == Some(Tile::Wall),
    });
    if !boxes_cannot_be_moved {
        for &boz in &boxes_that_have_to_be_moved {
            *get_mut(tiles, boz).unwrap() = Tile::Empty;
        }
        for &boz in &boxes_that_have_to_be_moved {
            let mut_tile_ref = get_mut(
                tiles,
                add(
                    boz,
                    match dir {
                        Direction::Up => (0, -1),
                        Direction::Down => (0, 1),
                        Direction::Left => (-1, 0),
                        Direction::Right => (1, 0),
                    },
                ),
            )
            .unwrap();
            *mut_tile_ref = Tile::Box;
        }
    }
    let box_count_after_mutation = tiles
        .iter()
        .map(|line| line.iter())
        .flatten()
        .filter(|&&tile| tile == Tile::Box)
        .count();
    assert_eq!(box_count_after_mutation, box_count_before_mutation);
    !boxes_cannot_be_moved
}

pub fn read_input(path: &str) -> (Vec<Vec<Tile>>, Vec<Direction>) {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    let mut line_iter = file_content.lines();

    let mut tiles = Vec::new();
    while let Some(line) = line_iter.next() {
        if line.trim().is_empty() {
            break;
        }
        let mut tile_line = Vec::new();
        for c in line.chars() {
            tile_line.push(match c {
                '#' => Tile::Wall,
                'O' => Tile::Box,
                '.' => Tile::Empty,
                '@' => Tile::Robot,
                _ => panic!("unexcpected character for tile"),
            });
            tile_line.push(match c {
                '#' => Tile::Wall,
                'O' => Tile::Empty,
                '.' => Tile::Empty,
                '@' => Tile::Empty,
                _ => panic!("unexcpected character for tile"),
            });
        }
        tiles.push(tile_line);
    }

    let mut directions = Vec::new();
    while let Some(line) = line_iter.next() {
        for c in line.chars() {
            directions.push(match c {
                '^' => Direction::Up,
                'v' => Direction::Down,
                '<' => Direction::Left,
                '>' => Direction::Right,
                _ => panic!("unexcpected character for direction"),
            })
        }
    }
    (tiles, directions)
}

fn add((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> (i32, i32) {
    (x0 + x1, y0 + y1)
}

fn get_mut(tiles: &mut Vec<Vec<Tile>>, (x, y): (i32, i32)) -> Option<&mut Tile> {
    if x < 0 || y < 0 {
        return None;
    }
    tiles
        .get_mut(y as usize)
        .and_then(|line| line.get_mut(x as usize))
}

fn get(tiles: &Vec<Vec<Tile>>, (x, y): (i32, i32)) -> Option<Tile> {
    if x < 0 || y < 0 {
        return None;
    }
    tiles
        .get(y as usize)
        .and_then(|line| line.get(x as usize))
        .cloned()
}

fn print_warehouse(tiles: &Vec<Vec<Tile>>) {
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            print!(
                "{}",
                match tiles[i][j] {
                    Tile::Robot => '@',
                    Tile::Wall => '#',
                    Tile::Box => '[',
                    Tile::Empty if get(tiles, (j as i32 - 1, i as i32)) == Some(Tile::Box) => ']',
                    Tile::Empty => '.',
                }
            );
        }
        println!()
    }
}

#[test]
fn test() {
    let (mut tiles, directions) = read_input("testinput.txt");
    print_warehouse(&tiles);
    let result = part2(&mut tiles, &directions);
    print_warehouse(&tiles);
    assert_eq!(9021, result);
}

#[test]
fn test_get_box() {
    let tiles = vec![
        vec![Tile::Box, Tile::Empty, Tile::Empty],
        vec![Tile::Empty, Tile::Empty, Tile::Empty],
    ];
    let results = [
        is_box(&tiles, (0, 0)),
        is_box(&tiles, (1, 0)),
        is_box(&tiles, (2, 0)),
        is_box(&tiles, (0, 1)),
        is_box(&tiles, (1, 1)),
        is_box(&tiles, (2, 1)),
    ];
    let expected = [Some((0, 0)), Some((0, 0)), None, None, None, None];
    assert_eq!(results, expected);
}

#[test]
fn  test_box_2() {
    let (tiles, _) = read_input("testinput.txt");
    assert_eq!(is_box(&tiles, (7, 4)), Some((6, 4)));
}
