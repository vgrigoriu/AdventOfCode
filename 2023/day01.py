from input import read_input


def first_and_last_digit(s: str) -> (int, int):
    digits_in_s = [c for c in s if c.isdigit()]
    return int(digits_in_s[0]), int(digits_in_s[-1])


def combine_digits(a: int, b: int) -> int:
    return a * 10 + b


def solve_day_01_part_1() -> int:
    lines = read_input(1)
    digits = [first_and_last_digit(line) for line in lines]
    numbers = [combine_digits(a, b) for a, b in digits]
    return sum(numbers)


def find_first_substring(s: str, substrings: list[str]) -> str:
    return min(
        substrings,
        key=lambda substring: s.find(substring) if s.find(substring) != -1 else len(s),
        default=None)


def find_last_substring(s: str, substrings: list[str]) -> str:
    return max(
        substrings,
        key=lambda substring: s.rfind(substring),
        default=None)


def first_and_last_digits_including_words(s: str) -> (int, int):
    digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4", "5", "6", "7",
              "8", "9"]
    first_digit = find_first_substring(s, digits)
    last_digit = find_last_substring(s, digits)
    return digits.index(first_digit) % 9 + 1, digits.index(last_digit) % 9 + 1


def solve_day_01_part_2() -> int:
    lines = read_input(1)
    digits = [first_and_last_digits_including_words(line) for line in lines]
    numbers = [combine_digits(a, b) for a, b in digits]
    return sum(numbers)


if __name__ == '__main__':
    print(solve_day_01_part_1())
    print(solve_day_01_part_2())
