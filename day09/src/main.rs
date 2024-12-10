#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Block {
    Free,
    Filled{ id: usize },
}

fn main() {
    println!("The solution to part 1 is {}", part1());
    println!("The solution to part 2 is {}", part2());
}

fn part1() -> i128 {
    let input = std::fs::read_to_string("input.txt").expect("could not read input file");
    let mut layout = disk_map_to_layout(&input);
    compact_file_system(&mut layout);
    checksum(&layout)
}

fn part2() -> i128 {
    let input = std::fs::read_to_string("input.txt").expect("could not read input file");
    let mut layout = disk_map_to_layout(&input);
    compact_file_system2(&mut layout);
    checksum(&layout)
}

fn disk_map_to_layout(diskmap: &str) -> Vec<Block> {
    let mut indicate_free_space = false;
    let mut current_id = 0;
    let mut layout = Vec::new();
    for c in diskmap.lines().next().expect("empty line 😔").chars() {
        let digit = c.to_digit(10).expect("got invalid number");
        if indicate_free_space {
            for _ in 0..digit {
                layout.push(Block::Free);
            }         
        } else {
            for _ in 0..digit {
                layout.push(Block::Filled { id: current_id });
            }
            current_id += 1;
        }
        indicate_free_space = !indicate_free_space;
    }
    layout
}

fn compact_file_system(layout: &mut [Block]) {
    for i in (0..layout.len()).rev() {
        if layout[i] == Block::Free {
            continue;
        }
        let free_block_idx = match layout.iter().position(|block| *block == Block::Free) {
            Some(fbi) => fbi,
            None => return,
        };
        if free_block_idx > i {
            return
        }
        layout.swap(i, free_block_idx);
    }
}

fn compact_file_system2(layout: &mut [Block]) {
    let mut i: i32  = (layout.len() - 1) as i32;
    while i >= 0 {
    // for i in (0..layout.len()).rev() {
        if layout[i as usize] == Block::Free {
            i -= 1;
            continue;
        }
        let block_size = get_block_size_backwards(i as usize, layout) as i32;
        let block_start = i - block_size + 1;
        let free_block_start = match find_free_block_of_size(block_size as usize, layout) {
            Some(fbs) => fbs as i32,
            None => {
                i -= block_size;
                continue
            },
        };
        if free_block_start > block_start {
            i -= block_size;
            continue;
        }
        for j in 0..block_size {
            layout.swap((block_start + j) as usize, (free_block_start + j) as usize);
        }
        i -= 1;
    }
}

//             *    *    *    *
// [ 0] [ 1] [ 2] [ 3] [ 4] [ 5] [ 6] [ 7]

fn find_free_block_of_size(size: usize, layout: &[Block]) -> Option<usize> {
    let mut i = 0;
    while i < layout.len() {
        let is_free = layout[i] == Block::Free;
        let block_size = get_block_size(i, layout);
        let block_start = i;
        i += block_size;
        if is_free && block_size >= size {
            return Some(block_start);
        }
    }
    None
}

fn get_block_size_backwards(idx: usize, layout: &[Block]) -> usize {
    let block = layout[idx];
    let mut size = 1;
    for i in (0..idx).rev() {
        if layout[i] != block {
            return size;
        }
        size += 1;
    }
    size
}

fn get_block_size(idx: usize, layout: &[Block]) -> usize {
    let block = layout[idx];
    let mut size = 1;
    for i in (idx+1)..layout.len() {
        if layout[i] != block {
            return size;
        }
        size += 1;
    }
    size
}

fn checksum(layout: &[Block]) -> i128 {
    let mut sum: i128 = 0;
    for (i, block) in layout.iter().enumerate() {
        sum += match block {
            Block::Filled{ id } => (i * id) as i128,
            Block::Free => 0,
        }
    }
    sum
}

#[test]
fn test_layout() {
    let expected = &[
        Block::Filled{ id: 0 },
        Block::Filled{ id: 0 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled{ id: 2 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Free,
        Block::Filled{ id: 4 },
        Block::Filled{ id: 4 },
        Block::Free,
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Free,
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Free,
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Free,
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 9 },
        Block::Filled{ id: 9 },
    ];
    let input = std::fs::read_to_string("testinput.txt").expect("could not read input file");
    assert_eq!(&disk_map_to_layout(&input), expected);
}

#[test]
fn test_compact() {
    let input = std::fs::read_to_string("testinput.txt").expect("could not read input file");
    let mut layout = disk_map_to_layout(&input);
    compact_file_system(&mut layout);
    let expected = &[
        Block::Filled{ id: 0 },
        Block::Filled{ id: 0 },
        Block::Filled{ id: 9 },
        Block::Filled{ id: 9 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 2 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 4 },
        Block::Filled{ id: 4 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
    ];
    assert_eq!(&layout, expected);
}

#[test]
fn test_checksum() {
    let input = std::fs::read_to_string("testinput.txt").expect("could not read input file");
    let mut layout = disk_map_to_layout(&input);
    compact_file_system(&mut layout);
    assert_eq!(checksum(&layout), 1928);
}

#[test]
fn teset_block_size() {
    let layout = &[
        Block::Filled { id: 1 }, // 0
        Block::Free, // 1
        Block::Free, // 2
        Block::Filled { id: 2 }, // 3
        Block::Filled { id: 2 }, // 4
        Block::Free, // 5
        Block::Free, // 6
        Block::Free, // 7
        Block::Free, // 8
        Block::Filled { id: 3 }, // 9
        Block::Filled { id: 3 }, // 10
        Block::Filled { id: 3 }, // 11
    ];
    let a = (get_block_size(0, layout),get_block_size(1, layout),get_block_size(3, layout),get_block_size(5, layout),get_block_size(9, layout));
    let b = (1, 2, 2, 4, 3);
    assert_eq!(a, b);
}

#[test]
fn teset_block_size_rev() {
    let layout = &[
        Block::Filled { id: 1 }, // 0
        Block::Free, // 1
        Block::Free, // 2
        Block::Filled { id: 2 }, // 3
        Block::Filled { id: 2 }, // 4
        Block::Free, // 5
        Block::Free, // 6
        Block::Free, // 7
        Block::Free, // 8
        Block::Filled { id: 3 }, // 9
        Block::Filled { id: 3 }, // 10
        Block::Filled { id: 3 }, // 11
    ];
    let a = (get_block_size_backwards(0, layout),get_block_size_backwards(2, layout),get_block_size_backwards(4, layout),get_block_size_backwards(8, layout),get_block_size_backwards(11, layout));
    let b = (1, 2, 2, 4, 3);
    assert_eq!(a, b);
    //  left: (2, 3, 3, 2, 3)
    // right: (1, 2, 2, 4, 3)
}

#[test]
fn test_free_block() {
    let layout = &[
        Block::Filled { id: 1 }, // 0
        Block::Free, // 1
        Block::Free, // 2
        Block::Filled { id: 2 }, // 3
        Block::Filled { id: 2 }, // 4
        Block::Free, // 5
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled { id: 3 },
        Block::Filled { id: 3 },
        Block::Filled { id: 3 },
    ];
    let (v0, v1) = (find_free_block_of_size(2, layout), find_free_block_of_size(3, layout));
    assert_eq!((v0, v1), (Some(1), Some(5)));
}

#[test]
fn test_compact2() {
    let input = std::fs::read_to_string("testinput.txt").expect("could not read input file");
    let mut layout = disk_map_to_layout(&input);
    compact_file_system2(&mut layout);

    let expected = &[
        Block::Filled{ id: 0 },
        Block::Filled{ id: 0 },
        Block::Filled{ id: 9 },
        Block::Filled{ id: 9 },
        Block::Filled{ id: 2 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 1 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Filled{ id: 7 },
        Block::Free,
        Block::Filled{ id: 4 },
        Block::Filled{ id: 4 },
        Block::Free,
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Filled{ id: 3 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Filled{ id: 5 },
        Block::Free,
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Filled{ id: 6 },
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Free,
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Filled{ id: 8 },
        Block::Free,
        Block::Free,
    ];
    assert_eq!(&layout, expected);
}
