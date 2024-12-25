import re
from dataclasses import dataclass

from input import read_aoc_input

type Registry = str


@dataclass
class Cpy:
    src: Registry | int
    dest: Registry

    def __str__(self) -> str:
        return f"cpy {self.src} {self.dest}"


@dataclass
class Inc:
    registry: Registry

    def __str__(self) -> str:
        return f"inc {self.registry}"


@dataclass
class Dec:
    registry: Registry

    def __str__(self) -> str:
        return f"dec {self.registry}"


@dataclass
class Jnz:
    test: Registry | int
    jump: int

    def __str__(self) -> str:
        return f"jnz {self.test} {self.jump}"


type Instr = Cpy | Inc | Dec | Jnz


def parse_instr(line: str) -> Instr:
    if m := re.match(r"cpy ([abcd]|-?\d+) ([abcd])", line):
        try:
            src = int(m[1])
        except ValueError:
            src = m[1]
        return Cpy(src, m[2])

    if m := re.match(r"inc ([abcd])", line):
        return Inc(m[1])

    if m := re.match(r"dec ([abcd])", line):
        return Dec(m[1])

    if m := re.match(r"jnz ([abcd]|-?\d+) (-?\d+)", line):
        try:
            test = int(m[1])
        except ValueError:
            test = m[1]
        return Jnz(test, int(m[2]))

    print(f"not yet parsing {line}")


class Computer:
    def __init__(self, instructions: list[Instr], c: int = 0) -> None:
        self.a = 0
        self.b = 0
        self.c = c
        self.d = 0

        # Instruction pointer
        self.ip = 0

        self.instructions = instructions

    def step(self) -> None:
        match self.instructions[self.ip]:
            case Inc(registry):
                setattr(self, registry, getattr(self, registry) + 1)
                self.ip += 1
            case Dec(registry):
                setattr(self, registry, getattr(self, registry) - 1)
                self.ip += 1
            case Cpy(src, dest):
                match src:
                    case int(n):
                        setattr(self, dest, n)
                    case str(r):
                        setattr(self, dest, getattr(self, r))
                self.ip += 1
            case Jnz(test, jump):
                match test:
                    case int(n):
                        value = n
                    case str(r):
                        value = getattr(self, r)
                if value:
                    self.ip += jump
                else:
                    self.ip += 1
            case _:
                raise ValueError(f"don't know to execute {self.instructions[self.ip]}")

    def is_halted(self) -> bool:
        return not (0 <= self.ip < len(self.instructions))

    def __str__(self) -> str:
        result = "a\tb\tc\td\n"
        result += f"{self.a}\t{self.b}\t{self.c}\t{self.d}\n\n"

        for i, instr in enumerate(self.instructions):
            if i == self.ip:
                result += "> "
            else:
                result += "  "
            result += f"{instr}\n"

        return result


instructions = read_aoc_input(parse_instr)
c1 = Computer(instructions)
while not c1.is_halted():
    c1.step()
print(c1.a)

c2 = Computer(instructions, 1)
while not c2.is_halted():
    c2.step()
print(c2.a)
