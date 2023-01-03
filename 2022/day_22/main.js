
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
    const parsedData = {
        map: new Map,
        instructions: []
    };

    for (let i = 0; i < data.length; i++) {
        parsedData.map.set(i, new Map);

        if (data[i] == "") {

            const directions = data[i + 1].replaceAll(/([A-Z])/g, "-$1-").split("-");
            parsedData.instructions = directions.map((val, index) => {
                if (index % 2 == 0)
                    return parseInt(val);
                return val;
            });
            break;
        }

        row = data[i].split("");

        for (let col = 0; col < row.length; col++) {

            if (row[col] == " ")
                continue;

            parsedData.map.get(i).set(col, row[col]);
        }

    }
    return parsedData;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    let currentPosition = [0, Math.min(...Array.from(parsedData.map.get(0).keys()))];
    let currentDirection = [0, 1];

    for (let i = 0; i < parsedData.instructions.length; i++) {

        if (i % 2 == 0) {
            // Odd means we are not rotating
            let missingMoves = parsedData.instructions[i];
            while (--missingMoves >= 0) {

                let nextPosition = [currentPosition[0] + currentDirection[0], currentPosition[1] + currentDirection[1]];
                // Check if next move is valid
                //  If cell exists, check if " " || "#"
                //  If not exists, then wrap around
                if (parsedData.map.has(nextPosition[0]) &&
                    parsedData.map.get(nextPosition[0]).has(nextPosition[1])) {

                    if (parsedData.map.get(nextPosition[0]).get(nextPosition[1]) != "#") {
                        currentPosition = nextPosition;
                        continue;
                    } else break;
                }
                else {
                    const shouldWrapRows = currentDirection[0] != 0;

                    if (shouldWrapRows) {
                        const bottomToTop = currentDirection[0] == 1;

                        if (bottomToTop) {
                            // Get the smallest row with col
                            for (let row = 0; row < currentPosition[0]; row++) {
                                if (parsedData.map.get(row).has(nextPosition[1])) {
                                    nextPosition[0] = row;
                                    break;
                                }
                            }
                        } else {
                            // Get the largest row with col
                            const allRowIndexs = Array.from(parsedData.map.keys());
                            //allRowIndexs.sort((a, b) => a - b);

                            for (let row = allRowIndexs.at(-1); row >= allRowIndexs.at(0); row--) {
                                if (parsedData.map.get(row).has(nextPosition[1])) {
                                    nextPosition[0] = row;
                                    break;
                                }
                            }
                        }
                    }
                    else {

                        const leftToRight = currentDirection[1] == 1;
                        const cols = Array.from(parsedData.map.get(nextPosition[0]).keys());
                        cols.sort((a, b) => a - b);

                        if (leftToRight) {
                            // Get the smallest col in row
                            nextPosition[1] = cols.at(0);

                        } else {
                            // Get the largest col in row
                            nextPosition[1] = cols.at(-1);
                        }
                    }

                    // Check if position is not " " or "#"
                    if (parsedData.map.get(nextPosition[0]).get(nextPosition[1]) != "#") {
                        currentPosition = nextPosition;
                        continue;
                    } else break;
                }
            }
        } else {
            // Rotate direction
            if (parsedData.instructions[i] == "R") {
                currentDirection = [currentDirection[1], -1 * currentDirection[0]]
            } else {
                currentDirection = [-currentDirection[1], currentDirection[0]]
            }
        }
    }


    let facing = 0;

    if (currentDirection[0] == 0 && currentDirection[1] == 1) facing = 0; // >
    else if (currentDirection[0] == 1 && currentDirection[1] == 0) facing = 1; // down
    else if (currentDirection[0] == 0 && currentDirection[1] == -1) facing = 2; // <
    else if (currentDirection[0] == -1 && currentDirection[1] == 0) facing = 3; // top
    else throw "Impossible";


    return 1000 * (currentPosition[0] + 1) + 4 * (currentPosition[1] + 1) + facing;
}


function getFace(currentPosition) {

    const [x, y] = currentPosition;

    if (x >= 0 && x < 50) {
        // 5 & 6
        if (y >= 100 && y < 150) {
            // 5
            return [0, 100, 49, 149, 5];
        }
        else if (y >= 150) {
            // 6
            return [0, 150, 49, 199, 6];
        } else throw "Impossible;"
    }

    if (x >= 50 && x < 100) {

        if (y >= 0 && y < 50) {
            return [50, 0, 99, 49, 1];
        }
        else if (y >= 50 && y < 100) {
            return [50, 50, 99, 99, 3];
        }
        else if (y >= 100 && y < 150) {
            return [50, 100, 99, 149, 4];
        }
        else throw "Impossible";
    }

    if (x >= 100 && x <= 149) {
        if (y >= 0 && y < 50) {
            return [100, 0, 149, 49, 2];
        }
        else throw "Impossible";
    }

    throw "Impossible";

}


const Borders = {
    Top: 0,
    Right: 1,
    Bottom: 2,
    Left: 3
};

function DirectionToBorder(direction) {

    const [dx, dy] = direction;

    if (dx == 0 && dy == -1)
        return Borders.Top;
    else if (dx == 0 && dy == 1)
        return Borders.Bottom;
    else if (dx == -1 && dy == 0)
        return Borders.Left;
    else return Borders.Right;
}

function getFollowingCoordinates(currentPosition, currentFace, currentDirection) {

    const border = DirectionToBorder(currentDirection);

    if (currentFace == 1) {

        switch (border) {
            case Borders.Top:
                return [0, 150 + currentPosition[0] - 50, [1, 0]];
            case Borders.Right:
                return [100, currentPosition[1], currentDirection];
            case Borders.Bottom:
                return [currentPosition[0], 50, currentDirection];
            case Borders.Left:
                return [0, 149 - currentPosition[1], [1, 0]];
        }
    }

    if (currentFace == 2) {

        switch (border) {
            case Borders.Top:
                return [currentPosition[0] - 100, 199, [0, -1]];
            case Borders.Right:
                return [99, 149 - currentPosition[1], [-1, 0]];
            case Borders.Bottom:
                return [99, 50 + currentPosition[0] - 100, [-1, 0]];
            case Borders.Left:
                return [99, currentPosition[1], currentDirection];
        }
    }

    if (currentFace == 3) {

        switch (border) {
            case Borders.Top:
                return [currentPosition[0], 49, currentDirection];
            case Borders.Right:
                return [100 + currentPosition[1] - 50, 49, [0, -1]];
            case Borders.Bottom:
                return [currentPosition[0], 100, currentDirection];
            case Borders.Left:
                return [currentPosition[1] - 50, 100, [0, 1]];
        }
    }

    if (currentFace == 4) {

        switch (border) {
            case Borders.Top:
                return [currentPosition[0], 99, currentDirection];
            case Borders.Right:
                return [149, 49 - (currentPosition[1] - 100), [-1, 0]];
            case Borders.Bottom:
                return [49, 150 + currentPosition[0] - 50, [-1, 0]];
            case Borders.Left:
                return [49, currentPosition[1], currentDirection];
        }
    }

    if (currentFace == 5) {

        switch (border) {
            case Borders.Top:
                return [50, 50 + currentPosition[0], [1, 0]];
            case Borders.Right:
                return [50, currentPosition[1], currentDirection];
            case Borders.Bottom:
                return [currentPosition[0], 150, currentDirection];
            case Borders.Left:
                return [50, (149 - currentPosition[1]), [1, 0]];
        }
    }


    if (currentFace == 6) {

        switch (border) {
            case Borders.Top:
                return [currentPosition[0], 149, currentDirection];
            case Borders.Right:
                return [50 + (currentPosition[1] - 150), 149, [0, -1]];
            case Borders.Bottom:
                return [100 + currentPosition[0], 0, currentDirection];
            case Borders.Left:
                return [50 + (currentPosition[1] - 150), 0, [0, 1]];
        }
    }


}

function getNextCoordinate(currentPosition, currentDirection) {

    // Get cuurent face
    const [xMin, yMin, xMax, yMax, faceId] = getFace(currentPosition);

    // Next position
    let newPosition = [currentPosition[0] + currentDirection[0], currentPosition[1] + currentDirection[1]];

    // Check if is a valid position for its face
    if (xMin <= newPosition[0] && newPosition[0] <= xMax && yMin <= newPosition[1] && newPosition[1] <= yMax)
        return [...newPosition, currentDirection];

    // Well, it's invalid. Which means it will wrap around other face
    return getFollowingCoordinates(newPosition, faceId, currentDirection);
}

function arraysEqual(a1, a2) {
    return JSON.stringify(a1) == JSON.stringify(a2);
}


function problem_2(parsedData) {

    let currentPosition = [Math.min(...Array.from(parsedData.map.get(0).keys())), 0];
    let currentDirection = [1, 0];

    for (let i = 0; i < parsedData.instructions.length; i++) {


        if (i % 2 == 0) {
            // Moves

            let amountOfMoves = parsedData.instructions[i];
            while (--amountOfMoves >= 0) {
                const [x, y, direction] = getNextCoordinate(currentPosition, currentDirection);
                if (parsedData.map.get(y).get(x) != "#") {
                    currentPosition = [x, y];
                    currentDirection = direction;
                } else break;
            }
        }
        else {
            const [dx, dy] = currentDirection;
            const instruction = parsedData.instructions[i];

            if (dx == 1 && dy == 0) {
                if (instruction == "R")
                    currentDirection = [0, 1];
                else currentDirection = [0, -1];
            } else if (dx == 0 && dy == 1) {
                if (instruction == "R")
                    currentDirection = [-1, 0];
                else currentDirection = [1, 0];
            } else if (dx == -1 && dy == 0) {
                if (instruction == "R")
                    currentDirection = [0, -1];
                else currentDirection = [0, 1];
            } else if (dx == 0 && dy == -1) {
                if (instruction == "R")
                    currentDirection = [1, 0];
                else currentDirection = [-1, 0]
            }
        }
    }

    let facing = 0;
    const [fx, fy] = currentDirection;

    if (fx == 1 && fy == 0) facing = 0; // >
    else if (fx == 0 && fy == 1) facing = 1; // down
    else if (fx == -1 && fy == 0) facing = 2; // <
    else if (fx == 0 && fy == -1) facing = 3; // top
    else throw "Impossible";

    // 1
    console.assert(arraysEqual(getFollowingCoordinates([50, 0], 1, [0, -1]), [0, 150, [1, 0]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([50, 49], 1, [0, 1]), [50, 50, [0, 1]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([99, 49], 1, [1, 0]), [100, 49, [1, 0]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([50, 49], 1, [-1, 0]), [0, 100, [1, 0]]), "left");

    // 2
    console.assert(arraysEqual(getFollowingCoordinates([100, 0], 2, [0, -1]), [0, 199, [0, -1]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([149, 49], 2, [0, 1]), [99, 99, [-1, 0]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([149, 0], 2, [1, 0]), [99, 149, [-1, 0]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([100, 49], 2, [-1, 0]), [99, 49, [-1, 0]]), "left");

    // 3
    console.assert(arraysEqual(getFollowingCoordinates([99, 50], 3, [0, -1]), [99, 49, [0, -1]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([50, 99], 3, [0, 1]), [50, 100, [0, 1]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([99, 99], 3, [1, 0]), [149, 49, [0, -1]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([50, 99], 3, [-1, 0]), [49, 100, [0, 1]]), "left");

    // 4
    console.assert(arraysEqual(getFollowingCoordinates([50, 100], 4, [0, -1]), [50, 99, [0, -1]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([99, 149], 4, [0, 1]), [49, 199, [-1, 0]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([99, 100], 4, [1, 0]), [149, 49, [-1, 0]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([50, 149], 4, [-1, 0]), [49, 149, [-1, 0]]), "left");

    // 5
    console.assert(arraysEqual(getFollowingCoordinates([0, 100], 5, [0, -1]), [50, 50, [1, 0]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([0, 149], 5, [0, 1]), [0, 150, [0, 1]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([49, 100], 5, [1, 0]), [50, 100, [1, 0]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([0, 149], 5, [-1, 0]), [50, 0, [1, 0]]), "left");

    // 6
    console.assert(arraysEqual(getFollowingCoordinates([0, 150], 6, [0, -1]), [0, 149, [0, -1]]), "top");
    console.assert(arraysEqual(getFollowingCoordinates([0, 199], 6, [0, 1]), [100, 0, [0, 1]]), "bottom");
    console.assert(arraysEqual(getFollowingCoordinates([49, 199], 6, [1, 0]), [99, 149, [0, -1]]), "right");
    console.assert(arraysEqual(getFollowingCoordinates([0, 150], 6, [-1, 0]), [50, 0, [0, 1]]), "left");

    return 1000 * (currentPosition[1] + 1) + 4 * (currentPosition[0] + 1) + facing;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))