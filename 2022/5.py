"""Advent of Code 2022 Day 5: Supply Stacks"""

import re


def create_stacks(stacks_str):
    *crates, num_stacks = stacks_str.split("\n")

    number_of_stacks = len(num_stacks.split())

    stacks = [[] for _ in range(number_of_stacks)]
    crates.reverse()
    for line in crates:
        for stackIdx, i in enumerate(range(1, len(crates[0]), 4)):
            if line[i].strip():
                stacks[stackIdx].append(line[i])

    return stacks


def parse_instruction(instruction):
    # move 1 from 2 to 1
    crates_to_move, from_stack, to_stack = map(int, re.findall(r"\d+", instruction))
    return crates_to_move, from_stack - 1, to_stack - 1


def init():
    with open("5.txt") as f:
        stacks_str, procedures = f.read().split("\n\n")

    stacks = create_stacks(stacks_str)

    return stacks, procedures.split("\n")


def main_1():
    """Main part 1"""
    stacks, procedures = init()
    for procedure in procedures:
        if procedure.strip():
            crates_to_move, from_stack, to_stack = parse_instruction(procedure)
            print(f"{crates_to_move=}, {from_stack=}, {to_stack=}")
            for _ in range(crates_to_move):
                stacks[to_stack].append(stacks[from_stack].pop())

    print(stacks)
    [print(stack[-1], end="") for stack in stacks]


def main_2():
    """Main part 2"""
    stacks, procedures = init()
    for procedure in procedures:
        if procedure.strip():
            crates_to_move, from_stack, to_stack = parse_instruction(procedure)
            print(f"{crates_to_move=}, {from_stack=}, {to_stack=}")
            stacks[to_stack].extend(stacks[from_stack][-crates_to_move:])
            stacks[from_stack] = stacks[from_stack][:-crates_to_move]

    print(stacks)
    [print(stack[-1], end="") for stack in stacks]


if __name__ == "__main__":
    # main_1()
    main_2()
