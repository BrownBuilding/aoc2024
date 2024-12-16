mod part1;
mod part2;

fn main() {
    let (mut tiles, directions) = part1::read_input("input.txt");
    println!("The solution to part 1 is {}", part1::part1(&mut tiles, &directions));
    let (mut tiles, directions) = part2::read_input("input.txt");
    println!("The solution to part 2 is {}", part2::part2(&mut tiles, &directions));
}
