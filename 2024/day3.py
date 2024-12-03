import re


def do(instr):
    return instr[3] == "do()"


def dont(instr):
    return instr[4] == "don't()"


def operands(instr):
    return int(instr[1]), int(instr[2])


with open("input/3.in") as f:
    memory_contents = f.read()

mul_regex = re.compile(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))")

instructions = mul_regex.findall(memory_contents)
sum_1 = 0
sum_2 = 0
enabled = True
for instr in instructions:
    if do(instr):
        enabled = True
    elif dont(instr):
        enabled = False
    else:
        a, b = operands(instr)
        sum_1 += a * b
        if enabled:
            sum_2 += a * b

print(sum_1)
print(sum_2)
