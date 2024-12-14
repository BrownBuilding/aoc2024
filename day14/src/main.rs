#[derive(Debug, PartialEq)]
struct Robot {
    position: Point,
    velocity: Point,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn main() {
    let mut input = read_input("input.txt");
    println!(
        "The solution to part 1 is {}",
        part1(&mut input, Point { x: 101, y: 103 }, 100)
    );
    let mut input = read_input("input.txt");
    println!(
        "The solution to part 2 is {}",
        part2(&mut input, Point { x: 101, y: 103 }) // part2(&mut input, Point { x: 11, y: 7 })
    );
}

fn check(input: &mut [Robot], dimensions: Point) -> bool {
    for i in 0..dimensions.x {
        'woah: for j in 0..dimensions.y {
            for k in 0..6 {
                let a0 = input.iter().find(|robot| robot.position == Point { x: i + k, y: j });
                if a0.is_none() {
                    continue 'woah;
                }
            }
            return true;
            // if let (Some(_), Some(_), Some(_)) = (a0, a1, a2) {
            // }
        }
    }
    false
}

fn part2(input: &mut [Robot], dimensions: Point) -> usize {
    let mut i = 0;
    loop {
        // if i % 372 == 0 {
        // let mut output = String::new();
        if check(input, dimensions) {
            for j in 0..dimensions.y {
                for i in 0..dimensions.x {
                    let count = input
                        .iter()
                        .filter(|robot| robot.position == Point { x: i, y: j })
                        .count();
                    if count == 0 {
                        // output.push('.');
                        print!("{}", '.')
                    } else {
                        // output.push_str(&count.to_string());
                        print!("{}", count)
                    }
                }
                // output.push('\n');
                println!("");
            }
            // println!("{}", output);
            println!(
                "These are the robots after {} second{} does this look like a christmans tree? (y/n)",
                i,
                if i == 1 { "" } else { "s" }
            );
            loop {
                let mut s = String::new();
                let _ = &std::io::stdin().read_line(&mut s).unwrap();
                match s.as_str().trim() {
                    "n" => break,
                    "y" => return i,
                    _ => break,
                };
                // }
            }
        } else {
            println!("checked {}", i);
        }
        do_step(input, dimensions);
        i += 1;
    }
}

fn part1(input: &mut [Robot], dimensions: Point, seconds: usize) -> usize {
    for _ in 0..seconds {
        do_step(input, dimensions);
    }
    let width_half = dimensions.x / 2;
    let height_half = dimensions.y / 2;

    let upper_left_quadrant = input
        .iter()
        .filter(|robot| robot.position.x < width_half && robot.position.y < height_half)
        .count();

    let upper_right_quadrant = input
        .iter()
        .filter(|robot| robot.position.x > width_half && robot.position.y < height_half)
        .count();

    let lower_left_quadrant = input
        .iter()
        .filter(|robot| robot.position.x < width_half && robot.position.y > height_half)
        .count();

    let lower_right_quadrant = input
        .iter()
        .filter(|robot| robot.position.x > width_half && robot.position.y > height_half)
        .count();

    println!(
        "{}, {}, {}, {}",
        upper_left_quadrant, upper_right_quadrant, lower_right_quadrant, lower_left_quadrant
    );
    upper_left_quadrant * upper_right_quadrant * lower_right_quadrant * lower_left_quadrant
}

fn do_step(input: &mut [Robot], dimensions: Point) {
    for robot in &mut *input {
        robot.position = robot.position + robot.velocity;
        if robot.position.x >= dimensions.x || robot.position.x < 0 {
            robot.position.x = robot.position.x.rem_euclid(dimensions.x);
        }
        if robot.position.y >= dimensions.y || robot.position.y < 0 {
            robot.position.y = robot.position.y.rem_euclid(dimensions.y);
        }
    }
}

fn read_input(path: &str) -> Vec<Robot> {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    file_content
        .lines()
        .map(|line| parse_robot(line).expect("could not parse line"))
        .collect()
}

fn parse_robot(line: &str) -> Option<Robot> {
    let rest = line.strip_prefix("p=")?;
    let (p0, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(",")?;
    let (p1, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(" v=")?;
    let (v0, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(",")?;
    let (v1, _rest) = parse_number(rest)?;
    Some(Robot {
        position: Point { x: p0, y: p1 },
        velocity: Point { x: v0, y: v1 },
    })
}

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn parse_number(line: &str) -> Option<(i64, &str)> {
    // i wish rust had regex support out of the box 😭
    let rest = line;
    let mut number = String::new();
    let (sign, mut rest) = {
        let (sign, new_rest) = rest.split_at_checked(1)?;
        if sign == "-" {
            (true, new_rest)
        } else {
            (false, rest)
        }
    };
    loop {
        let (d, r) = match rest.split_at_checked(1) {
            Some((d, r)) => (d, r),
            None => break,
        };
        let character = d.chars().next().unwrap();
        if character.is_digit(10) {
            number.push(character);
        } else {
            break;
        }
        rest = r;
    }
    number
        .parse()
        .ok()
        .map(|n: i64| (if sign { -1 } else { 1 } * n, rest))
}

#[test]
fn test_read_input() {
    let expected = [
        Robot {
            position: Point { x: 0, y: 4 },
            velocity: Point { x: 3, y: -3 },
        },
        Robot {
            position: Point { x: 6, y: 3 },
            velocity: Point { x: -1, y: -3 },
        },
        Robot {
            position: Point { x: 10, y: 3 },
            velocity: Point { x: -1, y: 2 },
        },
        Robot {
            position: Point { x: 2, y: 0 },
            velocity: Point { x: 2, y: -1 },
        },
        Robot {
            position: Point { x: 0, y: 0 },
            velocity: Point { x: 1, y: 3 },
        },
        Robot {
            position: Point { x: 3, y: 0 },
            velocity: Point { x: -2, y: -2 },
        },
        Robot {
            position: Point { x: 7, y: 6 },
            velocity: Point { x: -1, y: -3 },
        },
        Robot {
            position: Point { x: 3, y: 0 },
            velocity: Point { x: -1, y: -2 },
        },
        Robot {
            position: Point { x: 9, y: 3 },
            velocity: Point { x: 2, y: 3 },
        },
        Robot {
            position: Point { x: 7, y: 3 },
            velocity: Point { x: -1, y: 2 },
        },
        Robot {
            position: Point { x: 2, y: 4 },
            velocity: Point { x: 2, y: -3 },
        },
        Robot {
            position: Point { x: 9, y: 5 },
            velocity: Point { x: -3, y: -3 },
        },
    ];
    let result = read_input("testinput.txt");
    assert_eq!(&expected, result.as_slice());
}

#[test]
fn test_part1() {
    let mut input = read_input("testinput.txt");
    assert_eq!(part1(&mut input, Point { x: 11, y: 7 }, 100), 12)
}
