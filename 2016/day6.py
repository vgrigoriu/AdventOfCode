from collections import Counter

from input import read_aoc_input

lines = read_aoc_input()

columns = [
    [line[col_no] for line in lines] for col_no in range(len(lines[0]))
]

counters = [
    Counter(col) for col in columns
]

decoded = [c.most_common(1)[0][0] for c in counters]
print("".join(decoded))

decoded2 = [c.most_common()[-1][0] for c in counters]
print("".join(decoded2))