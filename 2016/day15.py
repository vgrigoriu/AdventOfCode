from dataclasses import dataclass


@dataclass
class Disc:
    positions: int
    start_position: int

    def position_at(self, t: int) -> int:
        return (self.start_position + t) % self.positions


def test_position_at() -> None:
    assert Disc(5, 4).position_at(0) == 4
    assert Disc(5, 4).position_at(1) == 0
    assert Disc(5, 4).position_at(19) == 3


# Disc #1 has 13 positions; at time=0, it is at position 11.
# Disc #2 has 5 positions; at time=0, it is at position 0.
# Disc #3 has 17 positions; at time=0, it is at position 11.
# Disc #4 has 3 positions; at time=0, it is at position 0.
# Disc #5 has 7 positions; at time=0, it is at position 2.
# Disc #6 has 19 positions; at time=0, it is at position 17.
d1 = Disc(13, 11)
d2 = Disc(5, 0)
d3 = Disc(17, 11)
d4 = Disc(3, 0)
d5 = Disc(7, 2)
d6 = Disc(19, 17)
d7 = Disc(11, 0)


def when_to_press(discs: list[Disc]) -> int:
    for t in range(20_000_000):
        for i, disc in enumerate(discs):
            if disc.position_at(t + 1 + i) != 0:
                break
        else:
            return t


print(when_to_press([d1, d2, d3, d4, d5, d6]))
print(when_to_press([d1, d2, d3, d4, d5, d6, d7]))