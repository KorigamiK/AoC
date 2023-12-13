from functools import reduce


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
    got_instances: list[int] = list(0 for _ in range(len(lines)))
    for line in lines:
        card, rest = line.split(": ")
        card = int(card[4:])
        winning, have = map(lambda x: map(int, x.split()), rest.split(" | "))
        winners = set(winning)
        got = reduce(lambda acc, x: acc + 1 if x in winners else acc, have, 0)
        got_instances[card - 1] = got

    score = [1] * len(got_instances)
    for c, wins in enumerate(got_instances):
        i, j = c + 1, c + 1 + wins
        score[i:j] = [x + score[c] for x in score[i:j]]
    print(sum(score))


if __name__ == "__main__":
    lines = open(0).read().strip().split("\n")
    # part1(lines)
    part2(lines)
