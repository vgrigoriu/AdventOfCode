from collections import defaultdict
from input import read_aoc_input


input = read_aoc_input(str)

reading_rules = True
rules = defaultdict(list)
updates = []
for line in input:
    if line.strip() == "":
        reading_rules = False
        continue

    if reading_rules:
        p1, p2 = line.strip().split("|")
        rules[int(p1)].append(int(p2))
    else:
        updates.append([int(page) for page in line.strip().split(",")])

part1 = 0
incorrect_updates = []
for update in updates:
    correct = True
    for i in range(len(update)):
        for j in range(i + 1, len(update)):
            if update[i] in rules[update[j]]:
                print(f"rule broken: {update[i]} before {update[j]}")
                incorrect_updates.append(update)
                correct = False
                break
        if not correct:
            break
    if correct:
        part1 += update[len(update) // 2]

print(part1)

part2 = 0
for update in incorrect_updates:
    print(update)
    for i in range(len(update)):
        for j in range(i + 1, len(update)):
            if update[i] in rules[update[j]]:
                print(f"rule broken: {i}-{update[i]} before {j}-{update[j]}")
                # swap
                update[i], update[j] = update[j], update[i]
    print(update)
    part2 += update[len(update) // 2]

print(part2)