def problem_1(data):
    return 0


def problem_2(data):
    return 0

if __name__ == '__main__':
    
    with open("./inputs/input.txt") as file:
        data = file.read().split("\n")

    print(f"Solution 1: {problem_1(data)}")
    print(f"Solution 2: {problem_2(data)}")
