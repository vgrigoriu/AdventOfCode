import functools
from dataclasses import dataclass

from input import read_input

day = 7


@dataclass(frozen=True)
class Hand:
    cards: list[int]
    bid: int

    def type(self) -> int:
        distinct_values = set(self.cards)
        if len(distinct_values) == 1:
            return 7
        if len(distinct_values) == 2:
            for value in distinct_values:
                # four of a kind...
                if self.cards.count(value) == 4:
                    return 6
                # ...or full house
                if self.cards.count(value) == 3:
                    return 5
        if len(distinct_values) == 3:
            for value in distinct_values:
                # three of a kind...
                if self.cards.count(value) == 3:
                    return 4
                # ...or two pairs
                if self.cards.count(value) == 2:
                    return 3
        if len(distinct_values) == 4:
            # one pair
            return 2
        # high card
        return 1


def compare(h1: Hand, h2: Hand) -> int:
    if h1.type() != h2.type():
        return h1.type() - h2.type()
    for card1, card2 in zip(h1.cards, h2.cards):
        if card1 != card2:
            return card1 - card2
    return 0


symbol_values = {
    "A": 14,
    "K": 13,
    "Q": 12,
    "J": 11,
    "T": 10,
    "9": 9,
    "8": 8,
    "7": 7,
    "6": 6,
    "5": 5,
    "4": 4,
    "3": 3,
    "2": 2
}


def symbols_to_numbers(symbols: str) -> list[int]:
    return [symbol_values[symbol] for symbol in symbols]


def solve_part_1():
    puzzle_input = read_input(day)
    cards_and_bids = [line.split(" ") for line in puzzle_input]
    hands = [Hand(symbols_to_numbers(cards), int(bid)) for cards, bid in cards_and_bids]
    sorted_hands = sorted(hands, key=functools.cmp_to_key(compare))
    scores = [hand.bid * (i + 1) for i, hand in enumerate(sorted_hands)]
    return sum(scores)


def solve_part_2():
    puzzle_input = read_input(day)
    return puzzle_input


if __name__ == '__main__':
    print(solve_part_1())
    print(solve_part_2())
