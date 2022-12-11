#!/usr/bin/python3

"""Advent of Code 2022 Day 2"""

# rock paper scissors
# 1    2     3


def get_input(file_path: str) -> list:
    with open(file_path, "r") as f:
        return f.read().splitlines()


def main_1():
    data = get_input("2.txt")
    score = 0
    for line in data:
        print(line)

        opponent, player = line.split()
        opponent_hand = get_oponent(opponent)
        player_hand = get_player(player)

        outcome = (player_hand - opponent_hand) % 3
        print(opponent_hand, player_hand, outcome)
        if outcome == 0:
            print("Draw")
            score += 3
        elif outcome == 1:
            print("Player wins")
            score += 6
        else:
            print("Opponent wins")
        score += player_hand

    printf(score)


def main_2():
    data = get_input("2.txt")
    score = 0
    for line in data:
        print(line)
        opponent, player = line.split()
        opponent_hand = get_oponent(opponent)
        game_outcome = get_player(player)  # 1 - loss, 2 - draw, 3 - win
        player_hand: int
        if game_outcome == 1:
            player_hand = (opponent_hand + 1) % 3 + 1
        elif game_outcome == 2:
            player_hand = opponent_hand
            score += 3
        else:
            player_hand = (opponent_hand - 1) % 3 + 1
            score += 6

        print(opponent_hand, player_hand, game_outcome)
        score += player_hand

    printf(score)


def get_oponent(hand: str) -> int:
    return ord(hand) % ord("A") + 1


def get_player(hand: str) -> int:
    return ord(hand) % ord("X") + 1


def get_outcome(player: int, opponent: int) -> int:
    return (player - opponent) % 3


if __name__ == "__main__":
    printf = print

    def print(*_):
        ...

    main_2()
