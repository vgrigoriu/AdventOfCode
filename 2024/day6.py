from input import read_aoc_input


input = read_aoc_input()


def find_guard(map):
    for i in range(0, len(map)):
        for j in range(0, len(map[i])):
            if map[i][j] == "^":
                return i, j

guard_x, guard_y = find_guard(input)
guard_direction = "up"
guard_gone = False
max_x = len(input)
max_y = len(input[0])


visited = {(guard_x, guard_y)}
visited_with_direction = {(guard_x, guard_y, guard_direction)}

in_loop = False

def move_guard(map):
    global guard_x, guard_y, guard_direction, guard_gone, in_loop
    next_x, next_y = guard_x, guard_y
    right_x, right_y = guard_x, guard_y
    if guard_direction == "up":
        guard_x -= 1
        next_x -= 2
        right_x -= 1
        right_y += 1
    elif guard_direction == "down":
        guard_x += 1
        next_x += 2
        right_x += 1
        right_y -= 1
    elif guard_direction == "left":
        guard_y -= 1
        next_y -= 2
        right_x -= 1
        right_y -= 1
    elif guard_direction == "right":
        guard_y += 1
        next_y += 2
        right_x += 1
        right_y += 1

    if (guard_x, guard_y, guard_direction) in visited_with_direction:
        in_loop = True
        return

    if (guard_x, guard_y) not in visited:
        visited.add((guard_x, guard_y))
        visited_with_direction.add((guard_x, guard_y, guard_direction))

    if next_x < 0 or next_x >= max_x or next_y < 0 or next_y >= max_y:
        guard_gone = True
        return

    if map[next_x][next_y] == "#":
        if guard_direction == "up":
            if map[right_x][right_y] == "#":
                # turn around
                guard_direction = "down"
            else:
                guard_direction = "right"
        elif guard_direction == "right":
            if map[right_x][right_y] == "#":
                # turn around
                guard_direction = "left"
            else:
                guard_direction = "down"
        elif guard_direction == "down":
            if map[right_x][right_y] == "#":
                guard_direction = "up"
            else:
                guard_direction = "left"
        elif guard_direction == "left":
            if map[right_x][right_y] == "#":
                guard_direction = "right"
            else:
                guard_direction = "up"

def print_map(map):
    for i in range(0, len(map)):
        for j in range(0, len(map[i])):
            if (i, j) == (guard_x, guard_y):
                if guard_direction == "up":
                    print("^", end="")
                elif guard_direction == "down":
                    print("v", end="")
                elif guard_direction == "left":
                    print("<", end="")
                elif guard_direction == "right":
                    print(">", end="")
            elif (i, j) in visited:
                print("X", end="")
            elif map[i][j] == "#":
                print("#", end="")
            else:
                print(".", end="")
        print()


while not guard_gone:
    move_guard(input)

print(len(visited))

def add_obstacle(map, x, y):
    new_map = []
    for i in range(0, len(map)):
        new_map.append("")
        for j in range(0, len(map[i])):
            if i == x and j == y:
                new_map[i] += "#"
            else:
                new_map[i] += map[i][j]

    return new_map

# simulate an obstacle in each possible position
possible_obstacles = []
for i in range(0, len(input)):
    for j in range(0, len(input[i])):
        if input[i][j] == ".":
            new_map = add_obstacle(input, i, j)
            guard_x, guard_y = find_guard(new_map)
            guard_direction = "up"
            guard_gone = False
            in_loop = False
            visited = {(guard_x, guard_y)}
            visited_with_direction = {(guard_x, guard_y, guard_direction)}
            while not guard_gone and not in_loop:
                move_guard(new_map)
            if in_loop:
                possible_obstacles.append((i, j))

print(len(possible_obstacles))