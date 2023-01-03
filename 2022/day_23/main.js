
const fs = require('fs');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    data = data.split("\n");
    const map = new Set;

    for (let i = 0; i < data.length; i++) {
        const row = data[i].split("");

        for (let j = 0; j < row.length; j++) {
            if (row[j] == "#") {
                map.add(`${j}.${i}`);
            }
        }
    }
    return map;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function print(data) {

    const xMin = -10;
    const xMax = 15;

    const yMin = -5;
    const yMax = 10;

    for (let i = yMin; i <= yMax; i++) {
        process.stdout.write("\n");
        for (let j = xMin; j <= xMax; j++) {

            if (data.has(`${j}.${i}`))
                process.stdout.write("#");
            else
                process.stdout.write(".");
        }
    }
    process.stdout.write("\n");
}

// Move elfs
function proposeMove(elf, data, round) {

    const [x, y] = elf[0].split(".").map(val => parseInt(val));

    const tryCoords = [
        // North
        [x - 1, y - 1],
        [x, y - 1],
        [x + 1, y - 1],
        // South
        [x - 1, y + 1],
        [x, y + 1],
        [x + 1, y + 1],
        // West
        [x - 1, y],
        // East
        [x + 1, y],
    ]

    const allEmpty = tryCoords.every(val => !data.has(`${val[0]}.${val[1]}`));
    if (allEmpty) {
        return [elf[0], {
            lastPosition: [x, y]
        }];
    }

    const lastMove = round % 4;
    const coordsToCheck = [
        {
            // North
            coords: [[x - 1, y - 1], [x, y - 1], [x + 1, y - 1]],
            destination: [x, y - 1]
        },
        {
            // South
            coords: [[x - 1, y + 1], [x, y + 1], [x + 1, y + 1]],
            destination: [x, y + 1]
        },
        {
            // West
            coords: [[x - 1, y - 1], [x - 1, y], [x - 1, y + 1]],
            destination: [x - 1, y]
        },
        {
            coords: [[x + 1, y - 1], [x + 1, y], [x + 1, y + 1]],
            destination: [x + 1, y]
        }
    ]

    for (let i = lastMove; i < lastMove + 4; i++) {

        if (coordsToCheck[i % 4].coords.every(([x, y]) => !data.has(`${x}.${y}`))) {

            const d = coordsToCheck[i % 4].destination;

            return [`${d[0]}.${d[1]}`, {
                lastPosition: [x, y]
            }]
        }
    }

    return [elf[0], {
        lastPosition: [x, y]
    }];
}

function calculateRectangle(currentState) {
    const finalPositions = Array.from(currentState.entries()).map(val => val[0].split("."));

    const xMin = Math.min(...finalPositions.map(val => parseInt(val[0])));
    const xMax = Math.max(...finalPositions.map(val => parseInt(val[0])));

    const yMin = Math.min(...finalPositions.map(val => parseInt(val[1])));
    const yMax = Math.max(...finalPositions.map(val => parseInt(val[1])));

    return Math.abs(xMax - xMin + 1) * Math.abs(yMax - yMin + 1) - finalPositions.length;
}

function problem_1(parsedData) {

    // Prepare initial State
    let currentState = new Map;
    for (const elf of parsedData.keys()) {
        currentState.set(elf,
            {
                lastPosition: elf.split(".").map(val => parseInt(val)),
            }
        );
    }

    // Rounds
    let round = 0;
    while (round != 10) {
        let possibilities = new Map;

        // First step
        for (const elf of currentState.entries()) {
            const nextElfPos = proposeMove(elf, currentState, round);

            if (!possibilities.has(nextElfPos[0]))
                possibilities.set(nextElfPos[0], []);

            possibilities.get(nextElfPos[0]).push(nextElfPos[1]);
        }

        // Second step
        let nextState = new Map;

        for (const possibility of possibilities.entries()) {

            if (possibility[1].length == 1) {
                nextState.set(possibility[0], possibility[1][0]);
            }

            else {

                const lPosition0 = `${possibility[1][0].lastPosition[0]}.${possibility[1][0].lastPosition[1]}`
                nextState.set(lPosition0, possibility[1][0]);

                const lPosition1 = `${possibility[1][1].lastPosition[0]}.${possibility[1][1].lastPosition[1]}`
                nextState.set(lPosition1, possibility[1][1]);

            }
        }

        // Update Current State
        currentState = nextState;
        round += 1;

    }

    // Get the rectangle
    return calculateRectangle(currentState)
}

function problem_2(parsedData) {

    // Prepare initial State
    let currentState = new Map;
    for (const elf of parsedData.keys()) {
        currentState.set(elf,
            {
                lastPosition: elf.split(".").map(val => parseInt(val)),
            }
        );
    }

    // Rounds
    let round = 0;
    while (true) {
        let possibilities = new Map;
        let someoneMoved = false;

        // First step
        for (const elf of currentState.entries()) {
            const nextElfPos = proposeMove(elf, currentState, round);

            if (!possibilities.has(nextElfPos[0]))
                possibilities.set(nextElfPos[0], []);

            possibilities.get(nextElfPos[0]).push(nextElfPos[1]);
        }

        // Second step
        let nextState = new Map;

        for (const possibility of possibilities.entries()) {
            const newCoordinates = possibility[0].split(".").map(val => parseInt(val));

            if (possibility[1].length == 1) {
                const oldCoordinates = possibility[1][0].lastPosition

                if (newCoordinates[0] != oldCoordinates[0] || newCoordinates[1] != oldCoordinates[1])
                    someoneMoved = true;

                nextState.set(possibility[0], possibility[1][0]);
            }

            else {

                const lPosition0 = `${possibility[1][0].lastPosition[0]}.${possibility[1][0].lastPosition[1]}`
                nextState.set(lPosition0, possibility[1][0]);

                const lPosition1 = `${possibility[1][1].lastPosition[0]}.${possibility[1][1].lastPosition[1]}`
                nextState.set(lPosition1, possibility[1][1]);
            }
        }

        // Update Current State
        currentState = nextState;
        round += 1;


        if (!someoneMoved) {
            print(currentState)
            break;
        }
    }

    return round
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))