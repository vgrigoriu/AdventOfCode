import re

from input import read_aoc_input


def compute_decompressed_length(data: str) -> int:
    position = 0
    decompressed_length = 0
    marker_pattern = re.compile(r"\((\d+)x(\d+)\)")
    while position < len(data):
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


def compute_decompressed_length_2(data: str) -> int:
    position = 0
    decompressed_length = 0
    marker_pattern = re.compile(r"\((\d+)x(\d+)\)")
    while position < len(data):
        if m := marker_pattern.search(data, position):
            # copy everything from current position until next marker
            decompressed_length += (m.start() - position)

            # take subsequent chars, decompress and repeat them
            length, times = (int(g) for g in m.groups())
            # decompress the next length bytes
            chunk = data[m.end():m.end()+length]
            decompressed_chunk_length = compute_decompressed_length_2(chunk)
            decompressed_length += (decompressed_chunk_length * times)

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
part2 = compute_decompressed_length_2(compressed_data)
print(part2)