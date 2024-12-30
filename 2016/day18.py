from input import read_aoc_input

starting_row = read_aoc_input()

room = [
    # pad with safe tiles
    list("." + starting_row + ".")
] + [
    # plus extra 399_999 rows, to make 400_000
    ["." for _ in range(len(starting_row) + 2)]
    for _ in range(399_999)
]

trap_patterns = ["^^.", ".^^", "^..", "..^"]

for i in range(1, 400_000):
    for j in range(1, len(starting_row) + 1):
        if "".join(room[i - 1][j - 1 : j + 2]) in trap_patterns:
            room[i][j] = "^"

safe_tiles = len([tile for line in room for tile in line if tile == "."])

print(safe_tiles - 800_000)
