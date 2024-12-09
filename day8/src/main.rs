fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(input));
    let input = read_input("input.txt");
    println!("The solution to part 2 is {}", part2(input));
}

fn part2(input: Vec<Vec<u8>>) -> usize {
    let mut antinodes = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == b'.' {
                continue;
            }

            for k in 0..input.len() {
                for l in 0..input[k].len() {
                    if input[k][l] == b'.' {
                        continue;
                    }

                    if i == k && j == l {
                        continue;
                    }

                    if input[i][j] != input[k][l] {
                        continue;
                    }

                    let (i, j, k, l) = (i as i32, j as i32, k as i32, l as i32);

                    // let a = (i, j);
                    // let b = (k, l);
                    let mut p = (i, j);
                    let d = (i - k, j - l);
                    loop {
                        assert_ne!(d, (0, 0));
                        if     p.0 < 0
                            || p.0 >= input.len() as i32
                            || p.1 < 0
                            || p.1 >= input[0].len() as i32
                        {
                            break;
                        }
                        if !antinodes.contains(&p) {
                            antinodes.push(p);
                        }
                        p = (p.0 + d.0, p.1 + d.1);
                    }

                    // println!(
                    //     "a = {:?}, b = {:?}, d = {:?}, p = {:?}, {:?}", 
                    //     a, b, d, p, antinodes.len(),
                    // );
                }
            }
        }
    }

    // for i in 0..input.len() {
    //     for j in 0..input[i].len() {
    //         let c = if antinodes.contains(&(i as i32, j as i32)) {
    //             '#'
    //         } else {
    //             input[i][j] as char
    //         };
    //         print!("{}", c);
    //     }
    //     println!("");
    // }

    antinodes.len()
    // let input = read_input(")
}

fn part1(input: Vec<Vec<u8>>) -> usize {
    let mut antinodes = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == b'.' {
                continue;
            }

            for k in 0..input.len() {
                for l in 0..input[k].len() {
                    if input[k][l] == b'.' {
                        continue;
                    }

                    if i == k && j == l {
                        continue;
                    }

                    if input[i][j] != input[k][l] {
                        continue;
                    }

                    let (i, j, k, l) = (i as i32, j as i32, k as i32, l as i32);

                    let a = (i, j);
                    // let b = (k, l);
                    let d = (i - k, j - l);
                    let p = (a.0 + d.0, a.1 + d.1);
                    // let q = (b.0 - d.0, b.1 - d.1);
                    if p.0 < 0
                        || p.0 >= input.len() as i32
                        || p.1 < 0
                        || p.1 >= input[0].len() as i32
                    {
                        continue;
                    }

                    if antinodes.contains(&p) {
                        continue;
                    }

                    antinodes.push(p);

                    // // println!(
                    // //     "a = {:?}, b = {:?}, d = {:?}, p = {:?}, {:?}", 
                    // //     a, b, d, p, antinodes.len(),
                    // // );
                    // for i in 0..input.len() {
                    //     for j in 0..input[i].len() {
                    //         let c = if antinodes.contains(&(i as i32, j as i32)) {
                    //             '#'
                    //         } else {
                    //             input[i][j] as char
                    //         };
                    //         print!("{}", c);
                    //     }
                    //     println!("");
                    // }
                }
            }
        }
    }

    antinodes.len()
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

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(input), 14);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(input), 34);
}
