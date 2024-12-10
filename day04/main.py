WORD_XMAS = "XMAS"
WORD_MAS = "MAS"

def main():
    # tests
    test_grid = read_grid("test_input.txt")
    part1_test_result = part1(test_grid);
    assert part1_test_result == 18, f"{part1_test_result} is not 18"
    part2_test_result = part2(test_grid);
    assert part2_test_result  == 9, f"{part2_test_result} is not 9"

    # actuall solutions
    grid = read_grid("input.txt")
    print(f"The solution to part 1 is {part1(grid)}")
    print(f"The solution to part 2 is {part2(grid)}")


def part1(grid: list[str]) -> int:
    count = 0
    for x in range(0, len(grid[0])):
        for y in range(0, len(grid)):
            count += parse_xmas(grid, x, y)
    return count


def part2(grid: list[str]) -> int:
    count = 0
    for x in range(0, len(grid[0])):
        for y in range(0, len(grid)):
            count += 1 if parse_x_mas(grid, x, y) else 0
    return count


def read_grid(path: str) -> list[str]:
    with open(path) as f:
        ret = [line for line in  f.read().splitlines()]
        width = len(ret[0])
        assert(all(map(lambda line: len(line) == width, ret)))
        return ret


def parse_xmas(input: list[str], x: int, y: int) -> int:
    """Part One"""
    directions = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    sum = 0
    for dir in directions:
        if parse_direction(input, x, y, dir[0], dir[1], WORD_XMAS):
            sum += 1
    return sum


def parse_direction(input: list[str], x: int, y: int, dx: int, dy: int, word: str) -> bool:
    """Gebe True wieder, wenn `word` am bei (`x`,`y`) anfängt und in Richtung
    (`dx`, `dy`) weiter geht."""
    for c in word:
        if x < 0 or x >= len(input[0]) or y < 0 or y >= len(input):
            return False
        if c != input[y][x]:
            return False
        x += dx
        y += dy
    return True


def parse_x_mas(grid: list[str], x: int, y: int) -> bool:
    """Part Two"""
    if not (parse_direction(grid, x, y, dx=1, dy=1, word=WORD_MAS)
            or parse_direction(grid, x + 2, y + 2, dx=-1, dy=-1, word=WORD_MAS)):
        return False
    if not (parse_direction(grid, x + 2, y, dx=-1, dy=1, word=WORD_MAS)
            or parse_direction(grid, x, y + 2, dx=1, dy=-1, word=WORD_MAS)):
        return False
    return True


if __name__ == '__main__':
    main()
