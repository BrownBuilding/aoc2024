use computer::{Computer, ComputerPair};

fn main() {
    let input = read_input("input.txt");
    println!("The solution to part 1 is {}", part1(&input));
    println!("The solution to part 2 is {}", part2(&input));
}

// Put all the structs in a separate module so I don't accidentally violate the invariance that
// the two computers in `ComputerPair` must always be sorted (ComputerPair::new(a, b) ==
// ComputerPair::new(b, a)).
mod computer {
    pub type Computer = [u8; 2];

    #[derive(Clone, Copy, Debug, PartialEq)]
    pub struct ComputerPair(Computer, Computer);

    impl ComputerPair {
        pub fn new(c0: Computer, c1: Computer) -> ComputerPair {
            match c0 < c1 {
                true => ComputerPair(c0, c1),
                false => ComputerPair(c1, c0),
            }
        }

        pub fn get(&self) -> (Computer, Computer) {
            (self.0, self.1)
        }

        pub fn contiains(&self, computer: Computer) -> bool {
            self.0 == computer || self.1 == computer
        }

        pub fn get_common(&self, rhs: &Self) -> Option<Computer> {
            let (c0, c1) = self.get();
            if rhs.contiains(c0) {
                Some(c0)
            } else if rhs.contiains(c1) {
                Some(c1)
            } else {
                None
            }
        }
    }
}

fn part1(connections: &[ComputerPair]) -> usize {
    let mut sets: Vec<[Computer; 3]> = Vec::new();
    for c in connections {
        let (c0, c1) = c.get();
        for o in connections {
            if o == c {
                continue;
            }
            if !o.contiains(c0) {
                continue;
            }
            // `c` and `o` have `c0` in common
            for h in connections {
                if h == o {
                    continue;
                }
                if h == c {
                    continue;
                }
                if !h.contiains(c1) {
                    continue;
                }

                // `c` and `h` have `c1` in common

                let c2 = match o.get_common(h) {
                    Some(c2) => c2,
                    None => continue,
                };

                // `h` and `o` have `c2` in common

                let mut ret = [c0, c1, c2];
                ret.sort();
                if !sets.contains(&ret) {
                    sets.push(ret);
                }
            }
        }
    }
    sets.retain(|s| s.iter().any(|c| c.starts_with(b"t")));
    sets.len()
}

fn part2(input: &[ComputerPair]) -> String {
    // construct list of all computers
    let computers = {
        let mut computers = Vec::<Computer>::new();
        for pair in input {
            let (c0, c1) = pair.get();
            if !computers.contains(&c0) {
                computers.push(c0);
            }
            if !computers.contains(&c1) {
                computers.push(c1);
            }
        }
        computers
    };

    // Construct some interconnected set involving Computer `c`
    let construct_interconnected_set = |&c: &Computer| {
        let mut set = Vec::<Computer>::new();
        set.push(c);
        for &d in &computers {
            if set.contains(&d) {
                continue;
            }
            // Check if there is an edge for every computer already in the set that connects the
            // new computer to it. If so, add the new computer to the set
            if set.iter().all(|&e| {
                let p_edge = ComputerPair::new(d, e);
                input.contains(&p_edge)
            }) {
                set.push(d)
            }
        }
        set
    };

    // For every computer, construct any interconnect set of computers involving it.
    // If done for every computer, it is guaranteed that the largest interconnected set is
    // generated.
    let mut largest_set = computers
        .iter()
        .map(construct_interconnected_set)
        .max_by_key(|set| set.len()) // get largest interconnected set
        .expect("no sets");
    largest_set.sort(); // sort computers in set alphabetically

    // Password
    let mut out = String::new();
    for &c in &*largest_set {
        out.push(char::from(c[0]));
        out.push(char::from(c[1]));
        out.push(',');
    }
    out.pop(); // Remove trailing comma
    out
}

fn read_input(path: &str) -> Vec<ComputerPair> {
    let file_content = std::fs::read_to_string(path).expect("could not read inputfile");
    file_content
        .lines()
        .map(|l| {
            let (c0, c1) = l.split_once('-').unwrap();
            let c0 = std::array::from_fn::<_, 2, _>(|d| c0.as_bytes()[d]);
            let c1 = std::array::from_fn::<_, 2, _>(|d| c1.as_bytes()[d]);
            ComputerPair::new(c0, c1)
        })
        .collect()
}

#[test]
fn test_part1() {
    let input = read_input("testinput.txt");
    assert_eq!(part1(&input), 7);
}

#[test]
fn test_part2() {
    let input = read_input("testinput.txt");
    assert_eq!(part2(&input), "co,de,ka,ta".to_string());
}

#[test]
fn test_common() {
    let a = ComputerPair::new(*b"hi", *b"ci");
    let b = ComputerPair::new(*b"bi", *b"ci");
    let results = [a.get_common(&b)];
    let expected = [Some(*b"ci")];
    assert_eq!(expected, results);
}
