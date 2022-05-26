
csv = []

with open("./inputs/input.txt", 'r') as f:
    for line in f:

        args = line.rstrip().split(" ")

        if args[0] == "inp":
            csv.append(["load w"])
        elif args[0] == "add":
            csv[-1].append("{} = {} + {}".format(args[1], args[1], args[2]))
        elif args[0] == "mul":
            csv[-1].append("{} = {} * {}".format(args[1], args[1], args[2]))
        elif args[0] == "div":
            csv[-1].append("{} = {} / {}".format(args[1], args[1], args[2]))
        elif args[0] == "mod":
            csv[-1].append("{} = {} % {}".format(args[1], args[1], args[2]))
        elif args[0] == "eql":
            csv[-1].append("{} = {} == {}".format(args[1], args[1], args[2]))
        else:
            raise "FAILED PARSED"

print([len(x) for x in csv])


for col in range(len(csv[0])):
    for row in range(len(csv)):
        print(csv[row][col], end=",")
    print()

