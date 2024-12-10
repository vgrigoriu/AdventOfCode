from input import read_aoc_input

def find_trails(line: int, col: int) -> set[tuple[int, int]]:
    if trail_heads[line][col] is not None:
        return trail_heads[line][col]

    start = data[line][col]
    if start == "9":
        # we're done
        trail_heads[line][col] = {(line, col)}
        return trail_heads[line][col]

    trail_heads_here = set()
    # up
    if line > 0 and ord(start) + 1 == ord(data[line - 1][col]):
        trail_heads_here = trail_heads_here.union(find_trails(line - 1, col))
    # down
    if line < len(data) - 1 and ord(start) + 1 == ord(data[line + 1][col]):
        trail_heads_here = trail_heads_here.union(find_trails(line + 1, col))
    # left
    if col > 0 and ord(start) + 1 == ord(data[line][col - 1]):
        trail_heads_here = trail_heads_here.union(find_trails(line, col - 1))
    # right
    if col < len(data[0]) - 1 and ord(start) + 1 == ord(data[line][col + 1]):
        trail_heads_here = trail_heads_here.union(find_trails(line, col + 1))
    
    trail_heads[line][col] = trail_heads_here
    return trail_heads[line][col]



def score(line: int, col: int) -> int:
    if data[line][col] != "0":
        return 0
    return len(find_trails(line, col))


data = read_aoc_input()
trail_heads = [
    [None for _ in line] for line in data
]
sum_of_scores = 0
for line in range(len(data)):
    for col in range(len(data[line])):
        s = score(line, col)
        if s > 0:
            sum_of_scores += score(line, col)


print(sum_of_scores)
