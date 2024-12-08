import re
from dataclasses import dataclass
from operator import itemgetter

from input import read_aoc_input


@dataclass
class Room:
    name: str
    sector_id: str
    checksum: str


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

part1 = 0
real_rooms = []
for room in rooms:
    frequencies = [[chr(i), 0] for i in range(ord("a"), ord("z") + 1)]
    for ch in room.name:
        if ch == "-":
            continue
        frequencies[ord(ch) - ord("a")][1] += 1

    frequencies.sort(key=itemgetter(1), reverse=True)
    common_letters = "".join(x[0] for x in frequencies[:5])
    if common_letters == room.checksum:
        part1 += room.sector_id
        real_rooms.append(room)

print(part1)

for room in real_rooms:
    real_name = rotate_string(room.name, room.sector_id)
    if "north" in real_name:
        print(room.sector_id)
        break
