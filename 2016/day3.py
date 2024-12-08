from input import read_aoc_input

input = read_aoc_input(lambda line: [int(n) for n in line.split()])


def is_triangle(sides: list[int]) -> bool:
    return (
        sides[0] + sides[1] > sides[2]
        and sides[1] + sides[2] > sides[0]
        and sides[2] + sides[0] > sides[1]
    )


part1 = len([sides for sides in input if is_triangle(sides)])

print(part1)


def transpose(input: list[list[int]]) -> list[list[int]]:
    return [
        sides
        for i in range(0, len(input), 3)
        for sides in [
            [input[i][0], input[i + 1][0], input[i + 2][0]],
            [input[i][1], input[i + 1][1], input[i + 2][1]],
            [input[i][2], input[i + 1][2], input[i + 2][2]],
        ]
    ]


transposed_input = transpose(input)
part2 = len([sides for sides in transposed_input if is_triangle(sides)])
print(part2)
