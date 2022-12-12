#!/usr/bin/python3

"""Advent of Code 2022 Day 4 solution"""


def main_1():
    score = 0
    with open("4.txt") as f:
        for line in f:

            elf1, elf2 = map(lambda x: list(map(int, x.split("-"))), line.split(","))
            if elf1[0] >= elf2[0] and elf1[1] <= elf2[1]:
                print("Elf 1 can see Elf 2")
                score += 1
            elif elf2[0] >= elf1[0] and elf2[1] <= elf1[1]:
                print("Elf 2 can see Elf 1")
                score += 1
            else:
                print("No elf can see the other")
    print(score)


def main_2():
    score = 0
    with open("4.txt") as f:
        for line in f:
            elf1, elf2 = map(lambda x: list(map(int, x.split("-"))), line.split(","))
            if elf1[0] <= elf2[0] <= elf1[1] or elf2[0] <= elf1[0] <= elf2[1]:
                print("Elf 1 and Elf 2 can see each other")
                score += 1
            else:
                print("No elf can see the other")
    print(score)


if __name__ == "__main__":
    main_2()
