
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
    return data.split("\n").map(val => val.split(" "));
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    // Helper functions

    const distance = function ([x1, y1], [x2, y2]) {

        if (x1 == x2) {
            // Same x
            return Math.abs(y2 - y1);
        }
        else if (y1 == y2) {
            // Same y
            return Math.abs(x2 - x1);
        }
        else {
            // Different x and y
            return Math.max(Math.abs(x2 - x1), Math.abs(y2 - y1));
        }

    }

    const getMoveStep = function (tail, head) {

        if (tail[0] == head[0])
            return [0, (head[1] - tail[1]) / Math.abs(head[1] - tail[1])];
        else if (tail[1] == head[1])
            return [(head[0] - tail[0]) / Math.abs(head[0] - tail[0]), 0];
        else {
            return [
                (head[0] - tail[0]) / Math.abs(head[0] - tail[0]),
                (head[1] - tail[1]) / Math.abs(head[1] - tail[1])
            ]
        }

    }

    // End Helper functions


    let uniquePositions = {};
    uniquePositions[[0, 0]] = true;

    let currentHead = [0, 0];
    let currentTail = [0, 0];

    parsedData.forEach(element => {

        const movement = [0, 0];
        // Get head movements
        switch (element[0]) {
            case "R":
                movement[0] += parseInt(element[1]);
                break;
            case "U":
                movement[1] += parseInt(element[1]);
                break;
            case "L":
                movement[0] -= parseInt(element[1]);
                break;
            case "D":
                movement[1] -= parseInt(element[1]);
                break;
            default:
                throw "Invalid movement!"
        }

        // Individually move head and tail

        const timeStep = [movement[0] / Math.abs(movement[0]) || 0, movement[1] / Math.abs(movement[1]) || 0];
        while (movement[0] != 0 || movement[1] != 0) {

            currentHead[0] += timeStep[0];
            currentHead[1] += timeStep[1];

            movement[0] -= timeStep[0];
            movement[1] -= timeStep[1];

            const head2Tail = distance(currentHead, currentTail);

            if (head2Tail <= 1) continue;

            const moveStepTail = getMoveStep(currentTail, currentHead);

            currentTail[0] += moveStepTail[0];
            currentTail[1] += moveStepTail[1];

            uniquePositions[currentTail] = true;
        }
    });

    return Object.keys(uniquePositions).length;
}

function problem_2(parsedData) {

    // Helper functions

    const distance = function ([x1, y1], [x2, y2]) {

        if (x1 == x2) {
            // Same x
            return Math.abs(y2 - y1);
        }
        else if (y1 == y2) {
            // Same y
            return Math.abs(x2 - x1);
        }
        else {
            // Different x and y
            return Math.max(Math.abs(x2 - x1), Math.abs(y2 - y1));
        }

    }

    const getMoveStep = function (tail, head) {

        if (tail[0] == head[0])
            return [0, (head[1] - tail[1]) / Math.abs(head[1] - tail[1])];
        else if (tail[1] == head[1])
            return [(head[0] - tail[0]) / Math.abs(head[0] - tail[0]), 0];
        else {
            return [
                (head[0] - tail[0]) / Math.abs(head[0] - tail[0]),
                (head[1] - tail[1]) / Math.abs(head[1] - tail[1])
            ]
        }

    }

    // End Helper functions


    let uniquePositions = {};
    uniquePositions[[0, 0]] = true;

    let currentHead = [0, 0];
    let currentTails = [
        [0, 0], // 1
        [0, 0], // 2
        [0, 0], // 3
        [0, 0], // 4
        [0, 0], // 5
        [0, 0], // 6
        [0, 0], // 7
        [0, 0], // 8
        [0, 0], // 9
    ];

    parsedData.forEach(element => {

        const movement = [0, 0];
        // Get head movements
        switch (element[0]) {
            case "R":
                movement[0] += parseInt(element[1]);
                break;
            case "U":
                movement[1] += parseInt(element[1]);
                break;
            case "L":
                movement[0] -= parseInt(element[1]);
                break;
            case "D":
                movement[1] -= parseInt(element[1]);
                break;
            default:
                throw "Invalid movement!"
        }

        // Individually move head and tail

        const timeStep = [movement[0] / Math.abs(movement[0]) || 0, movement[1] / Math.abs(movement[1]) || 0];
        while (movement[0] != 0 || movement[1] != 0) {

            currentHead[0] += timeStep[0];
            currentHead[1] += timeStep[1];

            movement[0] -= timeStep[0];
            movement[1] -= timeStep[1];

            for (let i = 0; i < currentTails.length; i++) {

                const target = i == 0 ? currentHead : currentTails[i - 1];
                const distanceToTarget = distance(target, currentTails[i]);

                if (distanceToTarget <= 1) break;

                const moveStepTail = getMoveStep(currentTails[i], target);
                currentTails[i][0] += moveStepTail[0];
                currentTails[i][1] += moveStepTail[1];
            }


            uniquePositions[currentTails[currentTails.length - 1]] = true;
        }
    });

    return Object.keys(uniquePositions).length;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))