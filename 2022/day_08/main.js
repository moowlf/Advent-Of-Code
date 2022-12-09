
const fs = require('fs');

//============================================================================
//                                                                   Helpers >
//============================================================================

Array.prototype.equal = function (arr) {

    if (this.length != arr.length) return false;

    for (let i = 0; i < this.length; i++) {
        if (this[i] != arr[i]) return false;
    }
    return true;
}

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    const parsedData = data.split("\n").map(val => val.split("").map(val => parseInt(val)));
    return parsedData;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    const visibleTrees = {};
    const ySize = parsedData.length;
    const xSize = parsedData[0].length;

    // |-> X
    // y

    // left to right

    for (let y = 1; y < ySize - 1; y++) {
        let currentTreeHeight = parsedData[y][0];
        for (let x = 1; x < xSize - 1; x++) {

            if (parsedData[y][x] > currentTreeHeight) {
                currentTreeHeight = parsedData[y][x];
                visibleTrees[[x, y]] = undefined;
            }
        }
    }

    // Right to left
    for (let y = 1; y < ySize - 1; y++) {
        let currentTreeHeight = parsedData[y][xSize - 1];

        for (let x = xSize - 2; x > 0; x--) {
            if (parsedData[y][x] > currentTreeHeight) {
                currentTreeHeight = parsedData[y][x];
                visibleTrees[[x, y]] = undefined;
            }
        }
    }

    // Top to bottom
    for (let x = 1; x < xSize - 1; x++) {

        let currentTreeHeight = parsedData[0][x];

        for (let y = 1; y < ySize - 1; y++) {
            if (parsedData[y][x] > currentTreeHeight) {
                currentTreeHeight = parsedData[y][x];
                visibleTrees[[x, y]] = undefined;
            }
        }
    }

    // Bottom to top
    for (let x = 1; x < xSize - 1; x++) {

        let currentTreeHeight = parsedData[ySize - 1][x];

        for (let y = ySize - 2; y > 0; y--) {
            if (parsedData[y][x] > currentTreeHeight) {
                currentTreeHeight = parsedData[y][x];
                visibleTrees[[x, y]] = undefined;
            }
        }
    }

    return Object.keys(visibleTrees).length + (xSize * 2 + (ySize - 2) * 2);
}

function problem_2(parsedData) {

    const xSize = parsedData[0].length;
    const ySize = parsedData.length;
    const Dirs = {
        "Left": 0,
        "Top": 1,
        "Right": 2,
        "Bottom": 3
    };

    const scenicScore = {};

    const directionIntoCoordinates = function (direction) {

        switch (direction) {
            case Dirs.Left:
                return [-1, 0];
            case Dirs.Top:
                return [0, -1];
            case Dirs.Right:
                return [1, 0];
            case Dirs.Bottom:
                return [0, 1];
            default:
                throw "Impossible direction found!"
        }
    }

    const limitDirection = function (treePos, direction) {
        switch (direction) {
            case Dirs.Left:
                return [0, treePos[1]];
            case Dirs.Top:
                return [treePos[0], 0];
            case Dirs.Right:
                return [xSize - 1, treePos[1]];
            case Dirs.Bottom:
                return [treePos[0], ySize - 1];
            default:
                throw "Impossible direction found!"
        }
    }

    const gazeIntoDirection = function (coords, direction, maxHeight) {

        const advanceIterator = directionIntoCoordinates(direction);
        const edge = limitDirection(coords, direction);

        let currentTrees = 0;

        while (!coords.equal(edge)) {

            const nextCoords = [
                coords[0] + advanceIterator[0],
                coords[1] + advanceIterator[1]
            ];

            const nextTreeHeight = parsedData[nextCoords[1]][nextCoords[0]];

            currentTrees += 1;

            if (nextTreeHeight >= maxHeight)
                break;

            coords = nextCoords;
        }

        return currentTrees;
    }

    // Create cells
    for (let x = 1; x < xSize - 1; x++) {
        for (let y = 1; y < ySize - 1; y++) {

            const currentCell = [x, y];
            const currentCellHeight = parsedData[y][x];

            // Check Left
            const lTrees = gazeIntoDirection(currentCell, Dirs.Left, currentCellHeight);

            // Check Top
            const tTrees = gazeIntoDirection(currentCell, Dirs.Top, currentCellHeight);

            // Check Right
            const rTrees = gazeIntoDirection(currentCell, Dirs.Right, currentCellHeight);

            // Check Bottom
            const bTrees = gazeIntoDirection(currentCell, Dirs.Bottom, currentCellHeight);

            scenicScore[[x, y]] = lTrees * tTrees * rTrees * bTrees;
        }
    }


    return Object.entries(scenicScore).sort((a, b) => b[1] - a[1])[0][1];
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))