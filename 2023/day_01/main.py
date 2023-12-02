

def problem_1(data):

    solution = 0

    for line in data:

        l = 0
        while l < len(line):
            if '0' <= line[l] <= '9':
                solution += int(line[l]) * 10
                break
            l += 1
        
        r = len(line) - 1
        while r >= 0:
            if '0' <= line[r] <= '9':
                solution += int(line[r])
                break
            r -= 1

    return solution


def problem_2(data):

    solution = 0

    for line in data:
        l = 0
        
        current = 0

        while l < len(line):

            if '0' <= line[l] <= '9':
                current += int(line[l]) * 10
                break
            elif line[l] == 'o' and l + 2 < len(line) and line[l+1:l+3] == 'ne':
                current += 10
                break

            elif line[l] == 't' and l + 2 < len(line) and line[l+1:l+3] == 'wo':
                current += 20
                break
            
            elif line[l] == 't' and l + 4 < len(line) and line[l+1:l+5] == 'hree':
                current += 30
                break

            elif line[l] == 'f' and l + 3 < len(line) and line[l+1:l+4] == 'our':
                current += 40
                break

            elif line[l] == 'f' and l + 3 < len(line) and line[l+1:l+4] == 'ive':
                current += 50
                break

            elif line[l] == 's' and l + 2 < len(line) and line[l+1:l+3] == 'ix':
                current += 60
                break

            elif line[l] == 's' and l + 4 < len(line) and line[l+1:l+5] == 'even':
                current += 70
                break

            elif line[l] == 'e' and l + 4 < len(line) and line[l+1:l+5] == 'ight':
                current += 80
                break

            elif line[l] == 'n' and l + 3 < len(line) and line[l+1:l+4] == 'ine':
                current += 90
                break

            l += 1

        r = len(line) - 1
        while r >= 0:

            if '0' <= line[r] <= '9':
                current += int(line[r])
                break
            elif line[r] == 'o' and r + 2 < len(line) and line[r+1:r+3] == 'ne':
                current += 1
                break

            elif line[r] == 't' and r + 2 < len(line) and line[r+1:r+3] == 'wo':
                current += 2
                break
            
            elif line[r] == 't' and r + 4 < len(line) and line[r+1:r+5] == 'hree':
                current += 3
                break

            elif line[r] == 'f' and r + 3 < len(line) and line[r+1:r+4] == 'our':
                current += 4
                break

            elif line[r] == 'f' and r + 3 < len(line) and line[r+1:r+4] == 'ive':
                current += 5
                break

            elif line[r] == 's' and r + 2 < len(line) and line[r+1:r+3] == 'ix':
                current += 6
                break

            elif line[r] == 's' and r + 4 < len(line) and line[r+1:r+5] == 'even':
                current += 7
                break

            elif line[r] == 'e' and r + 4 < len(line) and line[r+1:r+5] == 'ight':
                current += 8
                break

            elif line[r] == 'n' and r + 3 < len(line) and line[r+1:r+4] == 'ine':
                current += 9
                break

            r -= 1
        solution += current
    return solution


if __name__ == '__main__':
    
    with open("./inputs/input.txt") as file:
        data = file.read().split("\n")

    print(f"Solution 1: {problem_1(data)}")
    print(f"Solution 2: {problem_2(data)}")