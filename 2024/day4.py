import re


with open("input/4.in") as f:
    input = f.readlines()

def find_in_line(line, word):
    re_w = re.compile(word)
    re_re_w = re.compile(word[::-1])
    return len(re_w.findall(line)) + len(re_re_w.findall(line))

part1 = 0
# find word horizontally
for line in input:
    part1 = part1 + find_in_line(line, "XMAS")

# find word vertically
for i in range(len(input[0])):
    part1 = part1 + find_in_line("".join([line[i] for line in input]), "XMAS")

# find diagonally
for i in range(len(input)):
    diag = "".join([input[i+j][j] for j in range(len(input)-i)])
    part1 = part1 + find_in_line(diag, "XMAS")

for i in range(1, len(input)):
    diag = "".join([input[j][i+j] for j in range(len(input)-i)])
    part1 = part1 + find_in_line(diag, "XMAS")

for i in range(len(input)):
    diag = "".join([input[i-j][j] for j in range(i+1)])
    part1 = part1 + find_in_line(diag, "XMAS")

for i in range(1, len(input)):
    diag = "".join([input[len(input)-j-1][i+j] for j in range(len(input)-i)])
    part1 = part1 + find_in_line(diag, "XMAS")

print(part1)

part2 = 0

for i in range(1, len(input) - 1):
    for j in range(1, len(input[i]) - 1):
        if input[i][j] == "A":
            word1 = input[i-1][j-1] + input[i+1][j+1]
            word2 = input[i-1][j+1] + input[i+1][j-1]
            if word1 in ["MS", "SM"] and word2 in ["MS", "SM"]:
                part2 += 1

print(part2)