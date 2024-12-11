from collections import defaultdict
from input import read_aoc_input

data = read_aoc_input(lambda line: [int(x) for x in line.split()])

def blink(stone: int) -> list[int]:
    if stone == 0:
        return [1]
    stone_str = str(stone)
    if len(stone_str) % 2 == 0:
        return [int(stone_str[:len(stone_str)//2]), int(stone_str[len(stone_str)//2:])]
    return [stone * 2024]

three_times_memo = {}
hits_by_times = [0 for _ in range(75)]
def blink_n(stone: int, times: int):
    global max_in_cache
    #print(f"blink_n {stone} {times} times")
    if (stone, times) in three_times_memo:
        hits_by_times[times] += 1
        #print(f"hitting the cache for ({stone}, {times})")
        #print(three_times_memo[(stone, times)])
        return three_times_memo[(stone, times)]

    if times <= 3:
        #print(f"manually doing it for {stone} {times} times.")
        stones = [stone]
        for _ in range(times):
            stones = transform_once(stones)
        three_times_memo[(stone, times)] = stones
        #print(stones)
        return stones
    
    #print(f"recursing to {times-1} times")
    once = transform_once([stone])
    result = [stone for s in once for stone in blink_n(s, times - 1)]
    three_times_memo[(stone, times)] = result
    #print(result)
    return result

def transform_once(stones):
    return [new_stone for stone in stones for new_stone in blink(stone)]

# def transform_thrice(stones):
#     return [new_stone for stone in stones for new_stone in blink_three_times(stone)]

#for _ in range(25):
#    data = transform_once(data)
#print(len(data))

rez = blink_n(112, 50)

print(len(rez))
print(hits_by_times)
