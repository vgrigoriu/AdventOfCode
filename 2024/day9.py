from input import read_aoc_input

diskmap = read_aoc_input()

# build disk
# each entry is a block, 0 is empty otherwise it's the file id
# (need to subtract 1 when computin the checksum)

disk = []
is_file = True
file_id = 1
lengths = [int(i) for i in diskmap]
for l in lengths:
    if is_file:
        disk.extend([file_id] * l)
        file_id += 1
    else:
        disk.extend([0] * l)
    is_file = not is_file

# rearrange blocks
empty_block = 0
while disk[empty_block] > 0:
    empty_block += 1
last_block = len(disk) - 1
while disk[last_block] == 0:
    last_block -= 1

while empty_block < last_block:
    disk[empty_block] = disk[last_block]
    disk[last_block] = 0
    while disk[empty_block] > 0:
        empty_block += 1
    while disk[last_block] == 0:
        last_block -= 1

block_checksums = [i * (disk[i] - 1) for i in range(last_block + 1)]
print(sum(block_checksums))