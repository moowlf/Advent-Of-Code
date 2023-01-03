
const fs = require('fs');
const Heap = require('heap');

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

    const map = {
        origin: undefined,
        destination: undefined,
        x: data[0].length - 2,
        y: data.length - 2,
        winds: [],
        borders: new Set
    };


    for (let yIndex = 0; yIndex < data.length; yIndex++) {

        const row = data[yIndex].split("");

        // first row 
        if (yIndex == 0) {
            map.origin = [row.indexOf("."), yIndex];
            continue;
        }

        // last row
        if (yIndex == data.length - 1) {
            map.destination = [row.indexOf("."), yIndex];
            continue;
        }

        for (let xIndex = 1; xIndex < data[yIndex].length - 1; xIndex++) {
            if (row[xIndex] != ".") {
                map.winds.push({ coords: [xIndex, yIndex], char: row[xIndex] });
            }
        }
    }

    for (let y = 0; y < data.length; y++) {
        if (y == 0 || y == data.length - 1) {

            for (let x = 1; x <= map.x; x++) {
                if ((map.origin[0] != x || map.origin[1] != y) && (map.destination[0] != x || map.destination[1] != y))
                    map.borders.add(`${x}.${y}`);
            }
        }

        map.borders.add(`0.${y}`);
        map.borders.add(`${map.x + 1}.${y}`);

    }

    // Calculate minute to next storm
    const time_matrix = Array.from({ length: data.length * data[0].length });
    for (let i = 0; i < time_matrix.length; i++) {
        time_matrix[i] = {
            vertical: new Set,
            horizontal: new Set
        }
    }

    const convertToIndex = function (coord, map) {
        const x_size = map.x + 2;
        return coord[1] * x_size + coord[0];
    }

    // Horizontal
    for (let y = 0; y < data.length; y++) {
        // Get horizontal winds
        const horizontalWinds = map.winds.filter(val => val.coords[1] == y && (val.char == "<" || val.char == ">"));

        for (let x = 1; x <= map.x; x++) {

            const currentPositionIndex = convertToIndex([x, y], map);

            horizontalWinds.forEach(val => {

                let distance = undefined;

                if (x < val.coords[0] && val.char == ">")
                    distance = map.x + 1 - val.coords[0] + (x - 1);
                else if (x > val.coords[0] && val.char == "<") {
                    distance = val.coords[0] + (map.x - x);
                } else {
                    distance = Math.abs(val.coords[0] - x);
                }
                time_matrix[currentPositionIndex].horizontal.add(distance);
            });
        }
    }

    // Vertical
    for (let x = 0; x < data[0].length; x++) {
        const winds = map.winds.filter(val => val.coords[0] == x && (val.char == "v" || val.char == "^"));

        for (let y = 1; y <= map.y; y++) {

            const currentPositionIndex = convertToIndex([x, y], map);

            winds.forEach(wind => {

                let distance = undefined;

                if (y < wind.coords[1] && wind.char == "v") {
                    distance = map.y + 1 - wind.coords[1] + (y - 1);
                }
                else if (y > wind.coords[1] && wind.char == "^") {
                    distance = wind.coords[1] + (map.y - y);
                } else {
                    distance = Math.abs(wind.coords[1] - y);
                }

                time_matrix[currentPositionIndex].vertical.add(distance);
            });
        }
    }

    map.winds = time_matrix;
    return map;
}


//============================================================================
//                                                                      Main >
//============================================================================


function AStar(origin, destination, time, map) {

    // helpers

    function sleep(milliseconds) {
        var start = new Date().getTime();
        for (var i = 0; i < 1e7; i++) {
            if ((new Date().getTime() - start) > milliseconds) {
                break;
            }
        }
    }

    const reachedDestination = function (current, destination) {
        return current[0] == destination[0] && current[1] == destination[1];
    }

    const convertToIndex = function (coord, map) {
        const x_size = map.x + 2;
        return coord[1] * x_size + coord[0];
    }

    const availablePositon = function (position, time, map) {

        // Get Wind Cycles For Desired Position
        const windInMap = map.winds[convertToIndex(position, map)];

        // Horizontal
        const freeOfHorizontalWinds = !windInMap.horizontal.has(time % map.x);

        // Vertical
        const freeOfVerticalWinds = !windInMap.vertical.has(time % map.y);

        return freeOfHorizontalWinds && freeOfVerticalWinds;
    }

    const getNeighbours = function (node, map) {

        const [x, y] = node.coordinates;
        const possible_neighbours = [];

        const isBorder = function (coord, map) {
            return map.borders.has(`${coord[0]}.${coord[1]}`)
        }

        if (x - 1 >= 0 && !isBorder([x - 1, y], map)) {

            if (availablePositon([x - 1, y], node.time + 1, map)) {
                possible_neighbours.push([x - 1, y]);
            }
        }

        if (x + 1 <= map.x + 2 && !isBorder([x + 1, y], map)) {
            if (availablePositon([x + 1, y], node.time + 1, map)) {
                possible_neighbours.push([x + 1, y]);
            }
        }


        if (y - 1 >= 0 && !isBorder([x, y - 1], map)) {
            if (availablePositon([x, y - 1], node.time + 1, map)) {
                possible_neighbours.push([x, y - 1]);
            }
        }

        if (y + 1 <= map.y + 1 && !isBorder([x, y + 1], map)) {
            if (availablePositon([x, y + 1], node.time + 1, map)) {
                possible_neighbours.push([x, y + 1]);
            }
        }

        // Current position
        if (availablePositon([x, y], node.time + 1, map)) {
            possible_neighbours.push([x, y]);
        }


        return possible_neighbours;
    }

    const distance = function (from, to) {
        return Math.abs(to[0] - from[0]) + Math.abs(to[1] - from[1]);
    }

    // OpenSet & OpenSet
    const fScore = Array.from({ length: (map.x + 2) * (map.y + 2) }).fill(Number.MAX_SAFE_INTEGER);
    fScore[convertToIndex(origin, map)] = 0 + distance(origin, destination);

    const openSet = new Heap((a, b) => {
        aObj = JSON.parse(a);
        bObj = JSON.parse(b);
        return fScore[convertToIndex(aObj.coordinates, map)] - fScore[convertToIndex(bObj.coordinates, map)];
    });

    const closedSet = new Set;

    openSet.push(JSON.stringify({
        coordinates: origin,
        time: time
    }));

    // Cycle
    while (!openSet.empty()) {

        const current = JSON.parse(openSet.pop());
        // console.log(current)
        // sleep(1000)

        if (reachedDestination(current.coordinates, destination))
            return current.time;

        closedSet.add(JSON.stringify(current));

        // we're not there yet, so we need to check for neighbours
        const neighbours = getNeighbours(current, map);

        for (const neigh of neighbours) {

            const spentTime = current.time + 1;
            const indexOfNeighboor = convertToIndex(neigh, map);

            fScore[indexOfNeighboor] = spentTime //+ distance(neigh, destination);

            const neighbourObject = JSON.stringify({
                coordinates: neigh,
                time: spentTime
            });

            if (!openSet.has(neighbourObject) && !closedSet.has(neighbourObject))
                openSet.push(neighbourObject)
        }
    }
}

let parsedData = readInputFile("./inputs/input.txt");

function problem(parsedData) {

    const time_1 = AStar(parsedData.origin, parsedData.destination, 0, parsedData);
    const time_2 = AStar(parsedData.destination, parsedData.origin, time_1, parsedData);
    const time_3 = AStar(parsedData.origin, parsedData.destination, time_2, parsedData);

    console.log("Part 1: ", time_1);
    console.log("Part 2: ", time_3);
}
problem(parsedData)