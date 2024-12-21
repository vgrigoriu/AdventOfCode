import re
from dataclasses import dataclass, field
from typing import Optional

from input import read_aoc_input


@dataclass
class BotDestination:
    bot: int

@dataclass
class OutputDestination:
    output: int

type Destination = BotDestination | OutputDestination

def make_dest(dest_type: str, n: int) -> Destination:
    if dest_type == "bot":
        return BotDestination(n)
    if dest_type == "output":
        return OutputDestination(n)
    
    raise ValueError(f"don't know how to make destination of type {type}")

@dataclass
class BotGives:
    bot: int
    low: Destination
    high: Destination


@dataclass
class GoesTo:
    chip: int
    bot: int

type Instr = BotGives | GoesTo

gives = re.compile(
    r"bot (\d+) gives low to (bot|output) (\d+) and high to (bot|output) (\d+)"
)
goes = re.compile(
    r"value (\d+) goes to bot (\d+)"
)
def parse_instruction(line: str) -> Instr:
    if m := gives.match(line):
        bot = int(m[1])
        dest_low = m[2]
        low = int(m[3])
        dest_high = m[4]
        high = int(m[5])
        return BotGives(bot, make_dest(dest_low, low), make_dest(dest_high, high))

    if m := goes.match(line):
        value = int(m[1])
        bot = int(m[2])
        return GoesTo(value, bot)

    raise ValueError(f"No idea how to parse {line}")


instructions = read_aoc_input(parse_instruction)

@dataclass
class Bot:
    no: int
    chips: list[int] = field(default_factory=list)
    low: Destination | None = None
    high: Destination | None = None

    def receive(self, chip: int) -> None:
        if len(self.chips) == 2:
            raise OverflowError("bot already has 2 chips")

        self.chips.append(chip)
        if 17 in self.chips and 61 in self.chips:
            print(self.no)

    def is_ready(self) -> bool:
        return len(self.chips) == 2 and self.low is not None and self.high is not None


# max bot number seems to be 209
bots = [Bot(no) for no in range(210)]
# max output number seems to be 20
outputs = [[] for _ in range(21)]

def propagate(bot_no: int) -> None:
    bot = bots[bot_no]
    if not bot.is_ready():
        return

    # both destinations are there, proceed
    match bot.low:
        case BotDestination(bot_dest):
            bots[bot_dest].receive(min(bot.chips))
            propagate(bot_dest)
        case OutputDestination(out_dest):
            outputs[out_dest].append(min(bot.chips))

    match bot.high:
        case BotDestination(bot_dest):
            bots[bot_dest].receive(max(bot.chips))
            propagate(bot_dest)
        case OutputDestination(out_dest):
            outputs[out_dest].append(max(bot.chips))


for instr in instructions:
    match instr:
        case BotGives(bot_no, low, high):
            bots[bot_no].low = low
            bots[bot_no].high = high
            propagate(bot_no)
        case GoesTo(chip, bot_no):
            bots[bot_no].receive(chip)
            propagate(bot_no)

print(outputs[0][0] * outputs[1][0] * outputs[2][0])