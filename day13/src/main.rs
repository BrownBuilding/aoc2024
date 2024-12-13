#[derive(Debug, PartialEq, Clone, Copy)]
struct Arcade {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

fn main() {
    let input = read_input("input.txt");
    println!("The soution to part 1 is {:?}", part1(&input));
    println!("The soution to part 2 is {:?}", part2(&input));
}

fn part1(input: &[Arcade]) -> i64 {
    let mut sum = 0;
    for &arcade in input {
        if let Some(opt) = optimal(arcade) {
            sum += 3 * opt.0 + opt.1
        }
    }
    sum
}

fn part2(input: &[Arcade]) -> i64 {
    let mut sum = 0;
    for &arcade in input {
        let arcade = Arcade {
            prize: (
                arcade.prize.0 + 10000000000000,
                arcade.prize.1 + 10000000000000,
            ),
            ..arcade
        };
        if let Some(opt) = optimal(arcade) {
            sum += 3 * opt.0 + opt.1
        }
    }
    sum
}

fn optimal(
    Arcade {
        button_a,
        button_b,
        prize,
    }: Arcade,
) -> Option<(i64, i64)> {
    let (ax, ay) = button_a;
    let (bx, by) = button_b;
    let (px, py) = prize;
    let a = (px * by - py * bx) / (ax * by - ay * bx);
    if (px * by - py * bx) % (ax * by - ay * bx) != 0 {
        return None;
    }
    let b = (px - ax * a) / bx;
    if (px - ax * a) % bx != 0 {
        return None;
    };
    Some((a, b))
}

// fn optimal(
//     Arcade {
//         button_a,
//         button_b,
//         prize,
//     }: Arcade,
// ) -> Option<(i64, i64)> {
//     (0i64..=100i64)
//         .map(move |i| (0i64..=100i64).map(move |j| (i, j)))
//         .flatten()
//         .filter(|&(a, b)| {
//             let pos = (
//                 a * button_a.0 + b * button_b.0,
//                 a * button_a.1 + b * button_b.1,
//             );
//             pos == prize
//         })
//         .min_by_key(|(a, b)| 3 * a + b)
// }

fn read_input(path: &str) -> Vec<Arcade> {
    let file_content = std::fs::read_to_string(path).expect("could not read input file");
    let mut file_content = file_content.as_str();
    let mut arcades = Vec::new();
    loop {
        let (arcade, rest) = match parse_arcade(file_content) {
            Some((arcade, rest)) => (arcade, rest),
            None => break,
        };
        arcades.push(arcade);
        file_content = match rest
            .split_once('\n')
            .and_then(|(_, rest)| rest.split_once('\n'))
        {
            Some((_, rest)) => rest,
            None => break,
        }
    }
    arcades
}

fn parse_arcade(text: &str) -> Option<(Arcade, &str)> {
    // parse a button behavior
    let rest = text.strip_prefix("Button A: X+")?;
    let (a_x, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(", Y+")?;
    let (a_y, rest) = parse_number(rest)?;
    let (_, rest) = rest.split_once('\n')?;
    // parse b button behavior
    let rest = rest.strip_prefix("Button B: X+")?;
    let (b_x, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(", Y+")?;
    let (b_y, rest) = parse_number(rest)?;
    // parse price
    let (_, rest) = rest.split_once('\n')?;
    let rest = rest.strip_prefix("Prize: X=")?;
    let (price_x, rest) = parse_number(rest)?;
    let rest = rest.strip_prefix(", Y=")?;
    let (price_y, rest) = parse_number(rest)?;
    Some((
        Arcade {
            button_a: (a_x, a_y),
            button_b: (b_x, b_y),
            prize: (price_x, price_y),
        },
        rest,
    ))
}

fn parse_number(line: &str) -> Option<(i64, &str)> {
    let mut rest = line;
    let mut number = String::new();
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
    number.parse().ok().map(|n| (n, rest))
}

#[test]
fn test_parse_arcade() {
    let text = parse_arcade(
        "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400",
    );
    let expected = Arcade {
        button_a: (94, 34),
        button_b: (22, 67),
        prize: (8400, 5400),
    };
    assert_eq!(text, Some((expected, "")))
}

#[test]
fn test_read_input() {
    let results = &read_input("testinput.txt");
    let expected = [
        Arcade {
            button_a: (94, 34),
            button_b: (22, 67),
            prize: (8400, 5400),
        },
        Arcade {
            button_a: (26, 66),
            button_b: (67, 21),
            prize: (12748, 12176),
        },
        Arcade {
            button_a: (17, 86),
            button_b: (84, 37),
            prize: (7870, 6450),
        },
        Arcade {
            button_a: (69, 23),
            button_b: (27, 71),
            prize: (18641, 10279),
        },
    ]
    .as_slice();
    assert_eq!(expected, results);
}

#[test]
fn test_optimal() {
    let expected = (38, 86);
    let result = optimal(Arcade {
        button_a: (17, 86),
        button_b: (84, 37),
        prize: (7870, 6450),
    });
    assert_eq!(Some(expected), result);
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(480, part1(&input));
}
