
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
    return data.split("")
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/test.txt");


function print(data, y) {

    for (let i = y; i >= 0; i--) {
        process.stdout.write("#");
        for (let x = 0; x < 7; x++) {
            if (data[x].has(i)) {
                process.stdout.write("X");
            } else {
                process.stdout.write(".");
            }
        }
        process.stdout.write("#\n");
    }

    process.stdout.write("+-------+\n\n\n");

}

function moveCoordinates(ch, coordinates, map) {

    let move_coordinates = [-1, 0];
    if (ch == '>') move_coordinates = [1, 0];

    let newCoordinates = [];
    for (let i = 0; i < coordinates.length; i++) {

        const new_x = coordinates[i][0] + move_coordinates[0];
        if (new_x < 0 || new_x > 6) return coordinates;

        newCoordinates.push([new_x, coordinates[i][1]]);
    }

    // Check for collisions
    const hasCollision = newCoordinates.some(val => map[val[0]].has(val[1]));

    if (hasCollision) return coordinates;
    else return newCoordinates;
}

function getPieceAtPosition(positionIndex, yPosition) {

    switch (positionIndex) {

        case 0:
            return [
                [2, yPosition + 4], [3, yPosition + 4], [4, yPosition + 4], [5, yPosition + 4]
            ]
        case 1:
            return [
                [3, yPosition + 6], [2, yPosition + 5], [3, yPosition + 5], [4, yPosition + 5], [3, yPosition + 4]
            ]
        case 2:
            return [
                [4, yPosition + 6], [4, yPosition + 5], [2, yPosition + 4], [3, yPosition + 4], [4, yPosition + 4]
            ]
        case 3:
            return [
                [2, yPosition + 7], [2, yPosition + 6], [2, yPosition + 5], [2, yPosition + 4]
            ]
        case 4:
            return [
                [2, yPosition + 5], [3, yPosition + 5], [2, yPosition + 4], [3, yPosition + 4],
            ]
    }

}

function canMoveDown(currentPieceId, coordinates, map) {

    if (currentPieceId == 0) {
        // ----
        return coordinates.every(val => val[1] - 1 >= 0 && !map[val[0]].has(val[1] - 1));

    }
    else if (currentPieceId == 1) {
        // +
        const neededCoordinates = [coordinates[1], coordinates[3], coordinates[4]];
        return neededCoordinates.every(val => val[1] - 1 >= 0 && !map[val[0]].has(val[1] - 1));

    } else if (currentPieceId == 2) {
        // inverted L
        return coordinates.slice(2).every(val => val[1] - 1 >= 0 && !map[val[0]].has(val[1] - 1));

    }
    else if (currentPieceId == 3) {
        // |
        return coordinates[0][1] - 1 >= 0 && !map[coordinates[0][0]].has(coordinates[3][1] - 1);

    } else if (currentPieceId == 4) {
        // cube
        return coordinates.slice(2).every(val => val[1] - 1 >= 0 && !map[val[0]].has(val[1] - 1));

    }
    else throw "Impossible";

}

function getNewMaxY(coordinates) {
    return Math.max(...coordinates.map(val => val[1]));
}

function problem_1(parsedData) {

    // Build Map 
    const map = new Set;
    map[0] = new Set([]);
    map[1] = new Set([]);
    map[2] = new Set([]);
    map[3] = new Set([]);
    map[4] = new Set([]);
    map[5] = new Set([]);
    map[6] = new Set([]);

    let currentPieceId = 0;
    let currentMaxY = -1;
    let roundID = 0;
    let airID = 0;

    while (roundID < 2022) {

        // Move to next piece
        currentPieceId = currentPieceId % 5;
        let currentPosition = getPieceAtPosition(currentPieceId, currentMaxY);

        while (true) {

            // Move left or right
            currentPosition = moveCoordinates(parsedData[airID], currentPosition, map);

            // Update air
            airID = (airID + 1) % parsedData.length;

            // Check if can move down
            if (!canMoveDown(currentPieceId, currentPosition, map)) break;

            // Update Y coordinates
            currentPosition.forEach(val => val[1] -= 1);

        }

        // Add position to map
        currentPosition.forEach(val => {
            map[val[0]].add(val[1]);
        })

        // Update round ID && pieceID && y position
        roundID += 1;
        currentPieceId += 1;
        currentMaxY = Math.max(getNewMaxY(currentPosition), currentMaxY);
    }

    //print(map, currentMaxY + 3);
    return currentMaxY + 1;
}

function getTopProfile(mem) {

    const arr = [];

    for (let i = 0; i < 7; i++) {
        const val = Array.from(mem[i]);
        arr.push(Math.max(...val));
    }
    return arr;
}

function problem_2(parsedData) {


    // Build Map 
    const map = new Set;
    map[0] = new Set([]);
    map[1] = new Set([]);
    map[2] = new Set([]);
    map[3] = new Set([]);
    map[4] = new Set([]);
    map[5] = new Set([]);
    map[6] = new Set([]);

    let currentPieceId = 0;
    let currentMaxY = -1;
    let roundID = 0;
    let airID = 0;

    const seen_happening = {};
    let to_add = 0;

    while (roundID < 1_000_000_000_000) {

        // Move to next piece
        currentPieceId = currentPieceId % 5;
        let currentPosition = getPieceAtPosition(currentPieceId, currentMaxY);
        const profile = getTopProfile(map).map(val => currentMaxY - val);

        if (roundID > 2022 && roundID < 100_000) {

            if (!seen_happening[currentPieceId])
                seen_happening[currentPieceId] = {};

            if (!seen_happening[currentPieceId][airID]) {
                seen_happening[currentPieceId][airID] = {};
                seen_happening[currentPieceId][airID][profile] = [[roundID, currentMaxY]];
            }
            else {

                const arr = seen_happening[currentPieceId][airID][profile];

                const roundsDiff = roundID - arr[0][0];
                const heightDiff = currentMaxY - arr[0][1];

                const skippingRounds = Math.floor((1e12 - roundID) / roundsDiff);
                to_add = skippingRounds * heightDiff;
                roundID = 1e12 - (1e12 - roundID) % roundsDiff;
                continue;
            }
        }


        while (true) {

            // Move left or right
            currentPosition = moveCoordinates(parsedData[airID], currentPosition, map);

            // Update air
            airID = (airID + 1) % parsedData.length;

            // Check if can move down
            if (!canMoveDown(currentPieceId, currentPosition, map)) break;

            // Update Y coordinates
            currentPosition.forEach(val => val[1] -= 1);
        }

        // Add position to map
        currentPosition.forEach(val => {
            map[val[0]].add(val[1]);
        })

        // Update round ID && pieceID && y position
        roundID += 1;
        currentPieceId += 1;
        currentMaxY = Math.max(getNewMaxY(currentPosition), currentMaxY);
        //sleep(1000)
    }

    //print(map, currentMaxY + 3);
    return currentMaxY + 1 + to_add;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))