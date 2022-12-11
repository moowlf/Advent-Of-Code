
const fs = require('fs');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

// https://www.30secondsofcode.org/js/s/lcm
const lcm = (...arr) => {
    const gcd = (x, y) => (!y ? x : gcd(y, x % y));
    const _lcm = (x, y) => (x * y) / gcd(x, y);
    return [...arr].reduce((a, b) => _lcm(a, b));
};

class Monkey {

    constructor(id) {
        this.id = id;
        this.items = [];
        this.inspections = 0;

        this.operationArgs = [];
        this.operation = undefined;

        this.div = -1;
        this.throw_at_if_true = -1;
        this.throw_at_if_false = -1;
    }

    throwAt(val) {
        if (val % this.div == 0)
            return this.throw_at_if_true;
        return this.throw_at_if_false;
    }

}

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    data = data.split("\n");
    const monkeys = [];
    for (let i = 0; i < data.length; i++) {

        const id = data[i].match(/(\d+)/)[1];
        const monkey = new Monkey(parseInt(id));

        monkey.items = data[i + 1].match(/(\d+)/g).map(val => parseInt(val));

        monkey.operationArgs = data[i + 2].split("= ")[1].replaceAll(" ", "");

        monkey.operation = function (old) {
            return eval(this.operationArgs);
        }

        monkey.div = parseInt(data[i + 3].match(/(\d+)/)[1]);
        monkey.throw_at_if_true = parseInt(data[i + 4].match(/(\d+)/)[1]);
        monkey.throw_at_if_false = parseInt(data[i + 5].match(/(\d+)/)[1]);

        monkeys.push(monkey);
        i += 6;
    }

    return monkeys;
}


//============================================================================
//                                                                      Main >
//============================================================================

let monkeyData = readInputFile("./inputs/input.txt");
let monkeyData2 = readInputFile("./inputs/input.txt");

function problem_1(monkeys) {

    let currentMonkeyId = 0;
    let completedRound = 0;

    while (completedRound != 20) {

        for (let i = 0; i < monkeys[currentMonkeyId].items.length; i++) {

            let worryLevel = monkeys[currentMonkeyId].items[i];
            let newWorry = Math.floor(monkeys[currentMonkeyId].operation(worryLevel) / 3);
            const nextMonkey = monkeys[currentMonkeyId].throwAt(newWorry);
            monkeys[nextMonkey].items.push(newWorry);
        }

        monkeys[currentMonkeyId].inspections += monkeys[currentMonkeyId].items.length;
        monkeys[currentMonkeyId].items = [];

        currentMonkeyId = (currentMonkeyId + 1) % monkeys.length;

        if (currentMonkeyId == 0) completedRound++;
    }


    const inspections = monkeys.map(val => val.inspections).sort((a, b) => b - a);
    return inspections[0] * inspections[1];
}

function problem_2(monkeys) {
    let currentMonkeyId = 0;
    let completedRound = 0;

    let leastCommonMultiplier = lcm(...monkeys.map(val => val.div));

    while (completedRound != 10000) {

        for (let i = 0; i < monkeys[currentMonkeyId].items.length; i++) {

            let worryLevel = monkeys[currentMonkeyId].items[i];
            let newWorry = monkeys[currentMonkeyId].operation(worryLevel);
            const nextMonkey = monkeys[currentMonkeyId].throwAt(newWorry);
            monkeys[nextMonkey].items.push(newWorry % leastCommonMultiplier);
        }

        monkeys[currentMonkeyId].inspections += monkeys[currentMonkeyId].items.length;
        monkeys[currentMonkeyId].items = [];

        currentMonkeyId = (currentMonkeyId + 1) % monkeys.length;

        if (currentMonkeyId == 0) completedRound++;
    }


    const inspections = monkeys.map(val => val.inspections).sort((a, b) => b - a);
    return inspections[0] * inspections[1];
}


console.log("Problem #1: ", problem_1(monkeyData))
console.log("Problem #2: ", problem_2(monkeyData2))