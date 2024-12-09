import re
from dataclasses import dataclass
from operator import itemgetter

from input import read_aoc_input


@dataclass
class Room:
    name: str
    sector_id: str
    checksum: str

    def is_real(self) -> bool:
        frequencies = [[chr(i), 0] for i in range(ord("a"), ord("z") + 1)]
        for ch in self.name:
            if ch == "-":
                continue
            frequencies[ord(ch) - ord("a")][1] += 1

        # Stable sort, so alphabetical order is preserved in case of ties.
        frequencies.sort(key=itemgetter(1), reverse=True)
        common_letters = "".join(x[0] for x in frequencies[:5])

        return common_letters == self.checksum


def parse_room(line: str) -> Room:
    # bkwzkqsxq-tovvilokx-nozvyiwoxd-172[fstek]
    name, sector_id, checksum = re.search(r"([a-z-]+)-(\d+)\[([a-z]+)\]", line).groups()
    return Room(name, int(sector_id), checksum)


def rotate_letter(ch: str, n: int) -> str:
    if ch == "-":
        return " "
    return chr(ord("a") + ((ord(ch) - ord("a") + n) % (ord("z") + 1 - ord("a"))))


def rotate_string(s: str, n: int) -> str:
    return "".join(rotate_letter(ch, n) for ch in s)


rooms = read_aoc_input(parse_room)

real_rooms = [room for room in rooms if room.is_real()]
part1 = sum(room.sector_id for room in real_rooms)

print(part1)

for room in real_rooms:
    real_name = rotate_string(room.name, room.sector_id)
    if "north" in real_name:
        print(room.sector_id)
        break
