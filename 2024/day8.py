from input import read_aoc_input


def is_within_bounds(antinode: tuple[int, int]) -> bool:
    row, col = antinode
    return 0 <= row < len(map) and 0 <= col < len(map[row])


map = read_aoc_input()

# map from antena to positions, in order of appearance
visited_antenas = {}
antinodes = set()

for row in range(len(map)):
    for col in range(len(map[row])):
        if map[row][col] == ".":
            continue

        antenna = map[row][col]
        if antenna not in visited_antenas:
            # first time we see it, nothing else to do
            visited_antenas[antenna] = [(row, col)]
            continue

        prev_positions = visited_antenas[antenna]
        for prow, pcol in prev_positions:
            # compute antinodes between row, col and prow, pcol
            drow = row - prow
            dcol = col - pcol
            an1 = (prow - drow, pcol - dcol)
            if is_within_bounds(an1) and an1 not in antinodes:
                antinodes.add(an1)
            an2 = (row + drow, col + dcol)
            if is_within_bounds(an2) and an2 not in antinodes:
                antinodes.add(an2)

        visited_antenas[antenna].append((row, col))

print(len(antinodes))

visited_antenas = {}
antinodes = set()

for row in range(len(map)):
    for col in range(len(map[row])):
        if map[row][col] == ".":
            continue

        antenna = map[row][col]
        if antenna not in visited_antenas:
            # first time we see it, nothing else to do
            visited_antenas[antenna] = [(row, col)]
            continue

        prev_positions = visited_antenas[antenna]
        for prow, pcol in prev_positions:
            # compute antinodes between row, col and prow, pcol
            drow = row - prow
            dcol = col - pcol

            an1row, an1col = prow, pcol
            while is_within_bounds((an1row, an1col)):
                if (an1row, an1col) not in antinodes:
                    antinodes.add((an1row, an1col))
                an1row -= drow
                an1col -= dcol

            an2row, an2col = row, col
            while is_within_bounds((an2row, an2col)):
                if (an2row, an2col) not in antinodes:
                    antinodes.add((an2row, an2col))
                an2row += drow
                an2col += dcol

        visited_antenas[antenna].append((row, col))

print(len(antinodes))
