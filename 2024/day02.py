from input import read


def main():
    reports = read(2, Report)
    safe_reports = [report for report in reports if report.is_safe()]
    dampener_safe_reports = [report for report in reports if report.is_dampener_safe()]
    print(len(safe_reports))
    print(len(dampener_safe_reports))


class Report:
    def __init__(self, line: str):
        self.levels = [int(level) for level in line.split(" ")]

    def is_safe(self) -> bool:
        return self._are_safe(self.levels)

    def is_dampener_safe(self) -> bool:
        levels_with_each_removed = [
            self.levels[:i] + self.levels[i + 1 :] for i in range(len(self.levels))
        ]
        all_levels = [self.levels] + levels_with_each_removed
        return any(self._are_safe(levels) for levels in all_levels)

    def _are_safe(self, levels: list[int]) -> bool:
        diffs = [l1 - l2 for l1, l2 in zip(levels, levels[1:])]

        return all(1 <= diff <= 3 for diff in diffs) or all(
            -3 <= diff <= -1 for diff in diffs
        )


if __name__ == "__main__":
    main()
