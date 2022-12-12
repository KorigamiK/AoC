"""Advent of Code 2022 Day 3"""


def main():
    with open("3.txt") as file:
        data = file.read().splitlines()

    sum = 0
    for line in data:
        first = set(line[: len(line) // 2])
        second = set(line[len(line) // 2 :])
        for common in first.intersection(second):
            sum += (
                ord(common) - ord("a") + 1
                if common.islower()
                else 26 + ord(common) - ord("A") + 1
            )

    print(sum)


def main_2():
    with open("3.txt") as file:
        lines = file.read().splitlines()

    groups = [map(set, lines[i : i + 3]) for i in range(0, len(lines), 3)]
    total = 0
    for group in groups:
        common = set.intersection(*group)
        for char in common:
            total += (
                ord(char) - ord("a") + 1
                if char.islower()
                else 26 + ord(char) - ord("A") + 1
            )
    print(total)


if __name__ == "__main__":
    main_2()
