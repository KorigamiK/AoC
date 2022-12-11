"""Advent of Code 2022 Day 1 Solution"""

items = open("1.txt").read()


def main():
    print(
        sum(
            sorted(
                [
                    sum([int(x) for x in group.splitlines()])
                    for group in items.split("\n\n")
                ]
            )[::-1][:3]
        )
    )


if __name__ == "__main__":
    main()
