
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

    const caveProfile = data.split("\n").map(val => {
        val = val.split(" -> ");
        const all_coordinates = [];

        for (let i = 0; i < val.length - 1; i++) {

            const from = val[i].split(",").map(r => parseInt(r));
            all_coordinates.push(from);

            const to = val[i + 1].split(",").map(r => parseInt(r));
            all_coordinates.push(to);

            const step = [0, 0];
            let amountOfSteps = 0;

            if (from[0] == to[0]) {
                step[1] = to[1] > from[1] ? 1 : -1;
                amountOfSteps = Math.abs(to[1] - from[1]);
            } else if (from[1] == to[1]) {
                step[0] = to[0] > from[0] ? 1 : -1;
                amountOfSteps = Math.abs(to[0] - from[0]);
            } else {
                throw "Diagonal detected!";
            }

            let next = [from[0] + step[0], from[1] + step[1]];
            while (--amountOfSteps != 0) {
                all_coordinates.push(next);
                next = [next[0] + step[0], next[1] + step[1]];
            }
        }

        return all_coordinates;

    });

    const m = {};
    const flattedArr = caveProfile.flat();
    for (let i = 0; i < flattedArr.length; i++) {
        if (m[flattedArr[i][0]]) {
            m[flattedArr[i][0]].add(flattedArr[i][1]);
            continue;
        }

        m[flattedArr[i][0]] = new Set([flattedArr[i][1]]);
    }

    return m;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function solve(data, maxY) {

    let sand = {}
    const sandInitialLocation = [500, 0];

    const addToCave = (x, y) => {

        if (data[x]) {
            data[x].add(y);
        } else {
            data[x] = new Set([y])
        }
    };

    const addToSand = (x, y) => {

        if (sand[x]) {
            sand[x].add(y);
        } else {
            sand[x] = new Set([y])
        }
    };

    const downhill = function (x, y) {

        if (y > maxY) return "freefall";

        if (data[x] && data[x].has(y))
            return undefined;

        const down = downhill(x, y + 1);
        if (down == undefined) {

            const left = downhill(x - 1, y + 1);
            if (left == undefined) {

                const right = downhill(x + 1, y + 1);
                if (right == undefined) {
                    return [x, y];
                } else return right;
            } else return left;
        } else return down;

    }

    while (true) {

        // Get first position
        let possibleBlock = [sandInitialLocation[0], Math.min(...Array.from(data[sandInitialLocation[0]].values())) - 1];
        let found = downhill(possibleBlock[0], possibleBlock[1]);

        if (found == "freefall")
            break;

        addToCave(found[0], found[1]);
        addToSand(found[0], found[1]);
    }

    return Object.keys(sand).reduce((acc, val) => acc + sand[val].size, 0);
}

function solve_2(data, maxY) {

    let sand = {}
    const sandInitialLocation = [500, 0];

    const addToCave = (x, y) => {

        if (data[x]) {
            data[x].add(y);
        } else {
            data[x] = new Set([y])
        }
    };

    const addToSand = (x, y) => {

        if (sand[x]) {
            sand[x].add(y);
        } else {
            sand[x] = new Set([y])
        }
    };

    const downhill = function (x, y) {

        if (y >= maxY || (data[x] && data[x].has(y)))
            return undefined;

        const down = downhill(x, y + 1);
        if (down == undefined) {

            const left = downhill(x - 1, y + 1);
            if (left == undefined) {

                const right = downhill(x + 1, y + 1);
                if (right == undefined) {
                    return [x, y];
                } else return right;
            } else return left;
        } else return down;

    }

    while (true) {

        // Get first position
        let possibleBlock = [sandInitialLocation[0], Math.min(...Array.from(data[sandInitialLocation[0]].values())) - 1];

        if (possibleBlock[1] == -1) break;

        let found = downhill(possibleBlock[0], possibleBlock[1]);

        addToCave(found[0], found[1]);
        addToSand(found[0], found[1]);
    }

    return Object.keys(sand).reduce((acc, val) => acc + sand[val].size, 0);
}

function problem_1(data) {

    let maxY = Object.keys(data).reduce((acc, val) => {

        const max_v = Math.max(...data[val].values());
        if (max_v > acc) {
            return max_v
        }
        return acc;
    }, Number.MIN_SAFE_INTEGER);

    return solve(data, maxY);
}

function problem_2(data) {

    let maxY = Object.keys(data).reduce((acc, val) => {

        const max_v = Math.max(...data[val].values());
        if (max_v > acc) {
            return max_v
        }
        return acc;
    }, Number.MIN_SAFE_INTEGER);

    return solve_2(data, maxY + 2);
}


console.log("Problem #1: ", problem_1(structuredClone(parsedData)))
console.log("Problem #2: ", problem_2(parsedData))