#[derive(Clone, Copy, Debug)]
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
            })
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

pub fn part1(tiles: &mut Vec<Vec<Tile>>, directions: &[Direction]) -> usize {
    do_steps(tiles, directions);

    let mut sum = 0;
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            match tiles[i][j] {
                Tile::Box => {
                    sum += 100 * i + j;
                },
                _ => {}
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
                    Tile::Robot => return (i as i32, j as i32),
                    Tile::Wall => {}
                    Tile::Box => {}
                    Tile::Empty => {}
                }
            }
        }
        panic!("no robot :(")
    })();

    for dir in directions {
        let v = match dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };
        match get(tiles, add(v, robot_position)) {
            Some(Tile::Robot) => panic!("robot cannot collide with itself"),
            Some(Tile::Wall) => { /* do nothing */ }
            Some(Tile::Box) => {
                let mut box_pos = robot_position;
                loop {
                    box_pos = add(box_pos, v);
                    match get(tiles, box_pos) {
                        Some(Tile::Robot) => {
                            panic!("cannot push box into robot")
                        }
                        Some(Tile::Wall) => break,
                        Some(Tile::Box) => continue,
                        Some(Tile::Empty) => {
                            // set the tile of the original box to empty
                            *get_mut(tiles, add(v, robot_position)).unwrap() = Tile::Empty;
                            *get_mut(tiles, box_pos).unwrap() = Tile::Box;
                            *get_mut(tiles, robot_position).unwrap() = Tile::Empty;
                            robot_position = add(v, robot_position);
                            *get_mut(tiles, robot_position).unwrap() = Tile::Robot;
                            break
                        }
                        None => break,
                    }
                }
            }
            Some(Tile::Empty) => {
                *get_mut(tiles, robot_position).unwrap() = Tile::Empty;
                robot_position = add(v, robot_position);
                *get_mut(tiles, robot_position).unwrap() = Tile::Robot;
            }
            None => panic!("robot went off the grid"),
        }
        // println!("\nMove {}", match dir {
        //     Direction::Up => '^',
        //     Direction::Down => 'v',
        //     Direction::Left => '<',
        //     Direction::Right => '>',
        // });
        // print_grid(tiles);
    }
}

fn print_grid(tiles: &Vec<Vec<Tile>>) {
    for i in 0..tiles.len() {
        for j in 0..tiles[i].len() {
            print!("{}", match tiles[i][j] {
                Tile::Robot => '@',
                Tile::Wall => '#',
                Tile::Box => 'O',
                Tile::Empty => '.',
            });
        }
        println!();
    }
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

#[test]
fn test_part1() {
    let (mut tiles, directions) = read_input("testinput.txt");
    assert_eq!(part1(&mut tiles, &directions), 10092);
}
