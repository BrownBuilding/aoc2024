fn main() {
    let input = read_input("input.txt");
    let tiles = input.tiles.clone();
    let starting_position = input.starting_pos;
    let solutions = find_solutions(input);
    println!("the soulution to part 1 is {}", part1(&solutions));
    println!(
        "the soulution to part 2 is {}",
        part2(&solutions, starting_position, tiles)
    );
}

#[derive(Debug, Clone)]
struct Input {
    tiles: Vec<Vec<Tile>>,
    starting_pos: (i32, i32),
    end_pos: (i32, i32),
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    Explored {
        costs: [i32; 4], /* one cost for each direction */
    },
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Action {
    TurnClockwise,
    TurnCounterClockwise,
    Forward,
}

/// Represent a full or parital solution to the maze
#[derive(Clone, Debug, PartialEq)]
struct Solution {
    actions: Vec<Action>,
    end_pos: (i32, i32),
    end_dir: (i32, i32),
    cost: i32,
}

impl Solution {
    fn add(
        &self,
        action: Action,
        tiles: &mut Vec<Vec<Tile>>,
        // starting_pos: (i32, i32),
    ) -> Option<Self> {
        // Get final position and final direciton of the solution
        let (new_end_pos, new_end_dir) = move_dir(self.end_pos, self.end_dir, action);

        let new_cost = self.cost
            + match action {
                Action::TurnClockwise => 1000,
                Action::TurnCounterClockwise => 1000,
                Action::Forward => 1,
            };

        // check if the last tile of the solution has been explored in that direction and if so
        // check the cost of the solution that explored it at the time
        if action == Action::Forward {
            match get_mut(tiles, new_end_pos) {
                Some(Tile::Wall) | None => return None,
                Some(Tile::Explored { costs }) => {
                    let c = costs[dir_to_idx(new_end_dir)];
                    if new_cost > c {
                        return None;
                    }
                }
                _ => {}
            }
        }

        // filter solution out if it cannot be optimal (rotating in place)
        match (self.actions.last(), action) {
            (Some(Action::TurnCounterClockwise), Action::TurnClockwise) => return None,
            (Some(Action::TurnClockwise), Action::TurnCounterClockwise) => return None,
            (Some(Action::TurnCounterClockwise), Action::TurnCounterClockwise) => {
                // A solution that turns couter clockwise twice in a row could also have rotated
                // clockwise. No need to check both situtations
                return None;
            }

            _ => {}
        }
        // Turning counterclockwise three times is the same as turning clockwise once and vice
        // versa
        match (
            self.actions.get(self.actions.len().wrapping_sub(1)),
            self.actions.get(self.actions.len().wrapping_sub(2)),
            self.actions.get(self.actions.len().wrapping_sub(3)),
        ) {
            (
                Some(Action::TurnCounterClockwise),
                Some(Action::TurnCounterClockwise),
                Some(Action::TurnCounterClockwise),
            ) => return None,
            (
                Some(Action::TurnClockwise),
                Some(Action::TurnClockwise),
                Some(Action::TurnClockwise),
            ) => return None,
            _ => {}
        }

        // if action == Action::Forward {
        //     // make sure that a position does not appear twice in the solution
        //     if positions(&self.actions, starting_pos).any(|(pos, _dir)| pos == new_end_pos) {
        //         return None;
        //     }
        // }

        let mut new_action = self.actions.clone();
        new_action.push(action);

        if action == Action::Forward {
            let tile = get(tiles, new_end_pos).unwrap();
            *get_mut(tiles, new_end_pos).unwrap() = match tile {
                Tile::Empty => {
                    let mut costs = [i32::MAX, i32::MAX, i32::MAX, i32::MAX];
                    costs[dir_to_idx(new_end_dir)] = new_cost;
                    Tile::Explored { costs }
                }
                Tile::Explored { costs } => {
                    let mut costs = costs;
                    costs[dir_to_idx(new_end_dir)] = new_cost;
                    Tile::Explored { costs }
                }
                Tile::Wall => {
                    panic!("This is it luigi")
                }
            }
        }

        let solution = Self {
            actions: new_action,
            end_pos: new_end_pos,
            end_dir: new_end_dir,
            cost: new_cost,
        };

        Some(solution)
    }
}

fn move_dir(mut pos: (i32, i32), mut dir: (i32, i32), action: Action) -> ((i32, i32), (i32, i32)) {
    match action {
        Action::TurnClockwise => dir = clockwise(dir),
        Action::TurnCounterClockwise => dir = counter_clockwise(dir),
        Action::Forward => pos = add(dir, pos),
    };
    (pos, dir)
}

fn part1(solutions: &[Solution]) -> i32 {
    solutions[0].cost
}

fn part2(solutions: &[Solution], starting_pos: (i32, i32), tiles: Vec<Vec<Tile>>) -> usize {
    let mut seat_tiles = Vec::new();
    for s in solutions {
        for (pos, _dir) in positions(&s.actions, starting_pos) {
            if !seat_tiles.contains(&pos) {
                seat_tiles.push(pos);
            }
        }
    }
    for (i, line) in tiles.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            if let Some(_pos) = seat_tiles
                .iter()
                .rev()
                .find(|&&pos| pos == (j as i32, i as i32))
            {
                print!("{}", 'O');
            } else {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Explored { costs: _ } => ' ',
                    }
                );
            }
        }
        println!();
    }
    seat_tiles.len()
}

fn find_solutions(
    Input {
        mut tiles,
        starting_pos,
        end_pos,
    }: Input,
) -> Vec<Solution> {
    let mut solutions = Vec::new();
    let mut candidates = Vec::<Solution>::new();
    candidates.push(Solution {
        actions: vec![],
        end_pos: starting_pos,
        end_dir: (-1, 0),
        cost: 0,
    });
    let mut lowest_cost = None;
    let mut iterations = 0;

    loop {
        let can = if !candidates.is_empty() {
            let (min_index, _woah) = candidates
                .iter()
                .enumerate()
                .min_by(|a, b| a.1.cost.cmp(&b.1.cost))
                .unwrap();
            candidates.remove(min_index)
        } else {
            break;
        };

        iterations += 1;
        if iterations == 1_000 {
            iterations = 0;
            println!("{}, {}", iterations, candidates.len());
        }

        if let Some(lowest_cost) = lowest_cost {
            if can.cost > lowest_cost {
                continue;
            }
        }

        let mut add = |action: Action| match can.add(action, &mut tiles /*, starting_pos*/) {
            Some(c) => {
                if c.end_pos == end_pos {
                    lowest_cost = Some(c.cost);
                    println!("A soultion was found with cost {}", c.cost);
                    solutions.push(c);
                } else {
                    candidates.push(c);
                }
            }
            None => {}
        };

        add(Action::Forward);
        add(Action::TurnClockwise);
        add(Action::TurnCounterClockwise);
    }

    for s in &solutions {
        print_solution(&s, &tiles, starting_pos);
    }

    solutions
}

// Create an Iterator over all the positions of a solultion
fn positions<'a>(
    actions: &'a [Action],
    starting_pos: (i32, i32),
) -> impl Iterator<Item = ((i32, i32), (i32, i32))> + use<'a> {
    let mut pos = starting_pos;
    let mut dir = (-1, 0);
    actions.iter().map(move |action| {
        let (p, d) = move_dir(pos, dir, *action);
        pos = p;
        dir = d;
        (pos, dir)
    })
}

fn print_solution(solution: &Solution, tiles: &Vec<Vec<Tile>>, starting_pos: (i32, i32)) {
    for (i, line) in tiles.iter().enumerate() {
        for (j, tile) in line.iter().enumerate() {
            let positions: Vec<_> = positions(&solution.actions, starting_pos).collect();
            if let Some((_pos, dir)) = positions
                .iter()
                .rev()
                .find(|&&(pos, _dir)| pos == (j as i32, i as i32))
            {
                print!(
                    "{}",
                    match dir {
                        (0, -1) => '^',
                        (0, 1) => 'v',
                        (-1, 0) => '<',
                        (1, 0) => '>',
                        _ => panic!(),
                    }
                );
            } else {
                print!(
                    "{}",
                    match tile {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Explored { costs: _ } => ' ',
                    }
                );
            }
        }
        println!();
    }
}

fn counter_clockwise((dx, dy): (i32, i32)) -> (i32, i32) {
    match (dx, dy) {
        (-1, 0) => (0, 1),
        (0, 1) => (1, 0),
        (1, 0) => (0, -1),
        (0, -1) => (-1, 0),
        _ => panic!(),
    }
}

fn clockwise((dx, dy): (i32, i32)) -> (i32, i32) {
    match (dx, dy) {
        (-1, 0) => (0, -1),
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        _ => panic!(),
    }
}

fn dir_to_idx((dx, dy): (i32, i32)) -> usize {
    match (dx, dy) {
        (-1, 0) => 0,
        (0, -1) => 1,
        (1, 0) => 2,
        (0, 1) => 3,
        _ => panic!(),
    }
}

fn add((x0, y0): (i32, i32), (x1, y1): (i32, i32)) -> (i32, i32) {
    (x0 + x1, y0 + y1)
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

fn get_mut(tiles: &mut Vec<Vec<Tile>>, (x, y): (i32, i32)) -> Option<&mut Tile> {
    if x < 0 || y < 0 {
        return None;
    }
    tiles
        .get_mut(y as usize)
        .and_then(|line| line.get_mut(x as usize))
}

fn read_input(path: &str) -> Input {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    let mut starting_pos = None;
    let mut ending_pos = None;
    let mut tiles = Vec::new();
    for (i, line) in file_content.lines().enumerate() {
        let mut tile_line = Vec::new();
        for (j, ch) in line.chars().enumerate() {
            tile_line.push(match ch {
                '#' => Tile::Wall,
                '.' => Tile::Empty,
                'S' => {
                    starting_pos = Some((j as i32, i as i32));
                    Tile::Empty
                }
                'E' => {
                    ending_pos = Some((j as i32, i as i32));
                    Tile::Empty
                }
                _ => panic!("unexpected character in filie"),
            });
        }
        tiles.push(tile_line);
    }
    Input {
        tiles,
        starting_pos: starting_pos.expect("no starting position"),
        end_pos: ending_pos.expect("no starting position"),
    }
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    let solutions = find_solutions(input);
    let result0 = part1(&solutions);
    let input = read_input("testinput2.txt");
    let solutions = find_solutions(input);
    let result1 = part1(&solutions);
    assert_eq!((result0, result1), (7036, 11048));
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    let tiles = input.tiles.clone();
    let starting_pos = input.starting_pos;
    let solutions = find_solutions(input);
    let result0 = part2(&solutions, starting_pos, tiles.clone());
    let input = read_input("testinput2.txt");
    let tiles = input.tiles.clone();
    let starting_pos = input.starting_pos;
    let solutions = find_solutions(input);
    let result1 = part2(&solutions, starting_pos, tiles.clone());
    assert_eq!((result0, result1), (45, 64));
}

#[test]
fn test_finding_multiple_distinct_solutions() {
    let input = read_input("testinput.txt");
    let result0 = find_solutions(input);
    let input = read_input("testinput2.txt");
    let result1 = find_solutions(input);

    let check = |solutions: &[Solution]| {
        for i in 0..solutions.len() {
            for j in 0..solutions.len() {
                if i == j {
                    continue;
                }
                if solutions[i] == solutions[j] {
                    return false;
                }
            }
        }
        true
    };

    let expected = (true, true);
    let results = (check(&result0), check(&result1));
    assert_eq!(expected, results);
    // assert_ne!((result0, result1), (1, 1));
    // assert_ne!((result0, result1), (0, 0));
}

#[test]
fn test_mov_dir() {
    let (new_pos, new_dir) = move_dir((1, 13), (-1, 0), Action::TurnClockwise);
    assert_eq!((new_pos, new_dir), ((1, 13), (0, -1)));
    let (new_pos, new_dir) = move_dir((1, 13), (-1, 0), Action::Forward);
    assert_eq!((new_pos, new_dir), ((0, 13), (-1, 0)));
    let (new_pos, new_dir) = move_dir((1, 13), (-1, 0), Action::TurnCounterClockwise);
    assert_eq!((new_pos, new_dir), ((1, 13), (0, 1)));
}
