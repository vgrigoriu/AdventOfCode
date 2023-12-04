from dataclasses import dataclass

from input import read_input


@dataclass
class ScratchCard:
    winning_numbers: set[int]
    your_numbers: list[int]
    copies: int = 1

    def matching_numbers(self) -> int:
        return len([number for number in self.your_numbers if number in self.winning_numbers])

    def score(self) -> int:
        winners = [number for number in self.your_numbers if number in self.winning_numbers]
        return 2 ** (len(winners) - 1) if len(winners) > 0 else 0


@dataclass(frozen=True)
class ScratchCardsStack:
    cards: list[ScratchCard]

    def score_card(self, index: int) -> None:
        card = self.cards[index]
        for i in range(card.matching_numbers()):
            if index + 1 + i < len(self.cards):
                self.cards[index + 1 + i].copies += card.copies


def parse_scratch_card(line: str) -> ScratchCard:
    line = line.split(":")[1].strip()
    halves = line.split("|")
    winning_numbers = [int(number) for number in halves[0].strip().split()]
    your_numbers = [int(number) for number in halves[1].strip().split()]
    return ScratchCard(set(winning_numbers), your_numbers)


def solve_part_1(day: int):
    puzzle_input = read_input(day)
    scratch_cards = [parse_scratch_card(line) for line in puzzle_input]
    scores = [card.score() for card in scratch_cards]
    return sum(scores)


def solve_part_2(day: int):
    puzzle_input = read_input(day)
    scratch_cards = [parse_scratch_card(line) for line in puzzle_input]
    stack = ScratchCardsStack(scratch_cards)
    for i in range(len(stack.cards)):
        stack.score_card(i)
    copies = [card.copies for card in stack.cards]
    return sum(copies)


if __name__ == '__main__':
    day = 4
    print(solve_part_1(day))
    print(solve_part_2(day))
