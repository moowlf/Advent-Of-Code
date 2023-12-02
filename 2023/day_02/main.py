import re

def problem_1(data):

    id_sums = 0
    
    for game in data:

        # x color1, y color2; p color3; ....
        game_id, game_rounds = game.split(":")

        game_id = int(game_id[5:])

        # ["x color1, y color2", ...]
        game_rounds = game_rounds.split(";")

        # balls in bag
        seen_balls = {
            "red": 0,
            "green": 0,
            "blue": 0
        }

        for round in game_rounds:
            round_balls = round.split(",")

            for balls in round_balls:

                res = re.search(r'\s(\d+)\s(\w+)', balls)
                seen_balls[res.group(2)] = max(seen_balls[res.group(2)], int(res.group(1)))
        
        if seen_balls["red"] <= 12 and seen_balls["green"] <= 13 and seen_balls["blue"] <= 14:
            id_sums += game_id
    
    return id_sums


def problem_2(data):

    power = 0
    for game in data:

        # x color1, y color2; p color3; ....
        game_id, game_rounds = game.split(":")

        game_id = int(game_id[5:])

        # ["x color1, y color2", ...]
        game_rounds = game_rounds.split(";")

        # balls in bag
        seen_balls = {
            "red": 0,
            "green": 0,
            "blue": 0
        }

        for round in game_rounds:
            round_balls = round.split(",")

            for balls in round_balls:

                res = re.search(r'\s(\d+)\s(\w+)', balls)
                seen_balls[res.group(2)] = max(seen_balls[res.group(2)], int(res.group(1)))
        
        power += seen_balls["blue"] * seen_balls["green"] * seen_balls["red"]
    return power

if __name__ == '__main__':
    
    with open("./inputs/input.txt") as file:
        data = file.read().split("\n")

    print(f"Solution 1: {problem_1(data)}")
    print(f"Solution 2: {problem_2(data)}")
