from __future__ import annotations

from dataclasses import dataclass

from input import read_aoc_input


@dataclass
class Interval:
    start: int
    end: int

    def size(self) -> int:
        return self.end - self.start + 1

    def __sub__(self, other_interval: Interval) -> list[Interval]:
        start1 = self.start
        end1 = self.end
        start2 = other_interval.start
        end2 = other_interval.end

        # case 1:
        # start1 --- end1
        #                   start2 --- end2
        # case 2:
        #                     start1 --- end1
        # start2 --- end2
        if end1 < start2 or end2 < start1:
            return [self]

        # case 3:
        # start1 -------- end1
        #         start2 --------end2
        if start1 < start2 <= end1 <= end2:
            return [Interval(start1, start2 - 1)]

        # case 4:
        #          start1 -------- end1
        # start2 -----------end2
        if start2 <= start1 <= end2 < end1:
            return [Interval(end2 + 1, end1)]

        # case 5:
        #          start1 --- end1
        # start2 -------------------end2
        if start2 <= start1 <= end1 <= end2:
            return []

        # case 6:
        # start1 -----------------end1
        #         start2 --- end2
        if start1 < start2 <= end2 < end1:
            return [Interval(start1, start2 - 1), Interval(end2 + 1, end1)]

        raise ValueError(f"don't know how to subtract {other_interval} from {self}")


def test_interval_difference_disjunct() -> None:
    i1 = Interval(1, 10)
    i2 = Interval(23, 45)

    assert i1 - i2 == [i1]
    assert i2 - i1 == [i2]


def test_interval_difference_s1s2e1e2() -> None:
    i1 = Interval(0, 10)
    assert i1 - Interval(11, 100) == [Interval(0, 10)]
    assert i1 - Interval(10, 100) == [Interval(0, 9)]
    assert i1 - Interval(9, 100) == [Interval(0, 8)]
    assert i1 - Interval(9, 100) == [Interval(0, 8)]
    assert i1 - Interval(1, 100) == [Interval(0, 0)]
    assert i1 - Interval(7, 10) == [Interval(0, 6)]


def test_interval_difference_s2s1e2e1() -> None:
    i1 = Interval(10, 20)
    assert i1 - Interval(0, 9) == [Interval(10, 20)]
    assert i1 - Interval(0, 10) == [Interval(11, 20)]
    assert i1 - Interval(0, 12) == [Interval(13, 20)]
    assert i1 - Interval(0, 19) == [Interval(20, 20)]
    assert i1 - Interval(10, 17) == [Interval(18, 20)]


def test_interval_difference_s2s1e1e2() -> None:
    i1 = Interval(5, 9)
    assert i1 - Interval(0, 100) == []
    assert i1 - Interval(5, 100) == []
    assert i1 - Interval(0, 9) == []
    assert i1 - Interval(5, 9) == []

    i1 = Interval(3, 3)
    assert i1 - Interval(0, 100) == []
    assert i1 - Interval(3, 100) == []
    assert i1 - Interval(0, 3) == []
    assert i1 - Interval(3, 3) == []


def test_interval_difference_s1s2e2e1() -> None:
    i1 = Interval(2, 9)

    assert i1 - Interval(3, 3) == [Interval(2, 2), Interval(4, 9)]
    assert i1 - Interval(4, 7) == [Interval(2, 3), Interval(8, 9)]


@dataclass
class Intervals:
    intervals: list[Interval]

    def __sub__(self, other_interval: Interval) -> Intervals:
        return Intervals(
            [interval for i1 in self.intervals for interval in i1 - other_interval]
        )


def parse_interval(line: str) -> Interval:
    parts = line.split("-")
    return Interval(int(parts[0]), int(parts[1]))


input = read_aoc_input(parse_interval)

range = Intervals([Interval(0, 4294967295)])

for interval in input:
    range = range - interval

print(range.intervals[0].start)
print(sum(i.size() for i in range.intervals))
