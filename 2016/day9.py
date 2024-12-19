import re

from input import read_aoc_input


def compute_decompressed_length(data: str) -> int:
    position = 0
    decompressed_length = 0
    marker_pattern = re.compile(r"\((\d+)x(\d+)\)")
    while position < len(data) - 1:
        if m := marker_pattern.search(data, position):
            # copy everything from current position until next marker
            decompressed_length += (m.start() - position)
            # take subsequent chars and repeat them
            length, times = (int(g) for g in m.groups())
            decompressed_length += (length * times)
            # advance position
            position = m.end() + length
        else:
            # no marker until the end of data, we're done
            decompressed_length += (len(data) - position)
            position = len(data)
    
    return decompressed_length


compressed_data = read_aoc_input()

part1 = compute_decompressed_length(compressed_data)
print(part1)