#[derive(Clone, Copy, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let input = read_input("input.txt");
    println!("The soultion to part 1 is {}", part1(&input, 71, 1024));
    println!("Trying to solve part 2. This problem is brute-forcealbe and I did not choose appropriate data structures so this may take a minute ...");
    let part2_solution = part2(&input, 71);
    println!("The soultion to part 2 is \"{},{}\"", part2_solution.x, part2_solution.y);
}

fn part1(points: &[Point], size: usize, n_bytes: usize) -> usize {
    get_steps(&points[0..n_bytes], size).unwrap()
}

fn part2(points: &[Point], size: usize) -> Point {
    // Add one corrupted position every iteration and stop once there is no path to the exit.
    let mut i = 0;
    loop {
        match get_steps(&points[0..i], size) {
            Some(_) => i += 1,
            None => return points[i - 1],
        }
    }
}

fn get_steps(obstacles: &[Point], size: usize) -> Option<usize> {
    // Do a flood fill
    let mut levels = Vec::new();
    let mut l = Vec::new();
    l.push(Point { x: 0, y: 0 });
    levels.push(l);

    let mut i = 0;
    loop {
        let mut new_level = Vec::new();
        for &p in &levels[i] {
            for neigh in p.neighbors() {
                // check if `neigh` is in any of the previous levels
                if (0..i)
                    .map(|j| levels[j].iter())
                    .flatten()
                    .any(|&old_p| old_p == neigh)
                {
                    continue;
                }
                if neigh.x < 0 || neigh.x >= size as i32 || neigh.y < 0 || neigh.y >= size as i32 {
                    continue;
                }
                if obstacles.contains(&neigh) {
                    continue;
                }
                if new_level.contains(&neigh) {
                    continue;
                }
                // check if `neigh` is the exit position
                if neigh
                    == (Point {
                        x: size as i32 - 1,
                        y: size as i32 - 1,
                    })
                {
                    return Some(levels.len());
                }
                new_level.push(neigh);
            }
        }
        if new_level.len() == 0 {
            return None;
        }
        levels.push(new_level);
        i += 1;
    }
}

impl Point {
    fn neighbors(&self) -> [Point; 4] {
        [
            Point {
                x: self.x,
                y: self.y - 1,
            },
            Point {
                x: self.x,
                y: self.y + 1,
            },
            Point {
                x: self.x + 1,
                y: self.y,
            },
            Point {
                x: self.x - 1,
                y: self.y,
            },
        ]
    }
}

fn read_input(path: &str) -> Vec<Point> {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    file_content
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            }
        })
        .collect()
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    let result = part1(&input, 7, 12);
    let expected = 22;
    assert_eq!(result, expected);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    let result = part2(&input, 7);
    let expected = Point { x: 6, y: 1 };
    assert_eq!(result, expected);
}
