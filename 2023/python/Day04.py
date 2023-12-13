def part1(lines: list[str]):
    sum_ = 0
    for line in lines:
        winning, have = map(
            lambda x: map(int, x.split()), line.split(": ")[1].split(" | ")
        )
        winners = set(winning)
        ans = 0
        for i in have:
            if i in winners:
                ans += 1
        sum_ += 0 if ans == 0 else 1 << (ans - 1)

    print(sum_)


def part2(lines: list[str]):
    for line in lines:
        card, rest = line.split(": ")
        card = int(card[4:])
        winning, have = map(lambda x: map(int, x.split()), rest.split(" | "))
        print()


if __name__ == "__main__":
    lines = open(0).read().strip().split("\n")
    part1(lines)
    part2(lines)
