
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
    const cubesArray = data.split("\n").map(val => val.split(",").map(num => parseInt(num)));


    const cubes = new Set;

    for (const cube of cubesArray) {
        cubes.add(`${cube[0]}-${cube[1]}-${cube[2]}`)
    }

    return cubes;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function buildKey(coords) {
    return `${coords[0]}-${coords[1]}-${coords[2]}`
}

function problem_1(cubes) {

    let numFaces = 0;

    for (const cube of cubes) {

        const coordinates = cube.split("-").map(val => parseInt(val));

        // Check top
        if (!cubes.has(buildKey([coordinates[0], coordinates[1], coordinates[2] + 1]))) {
            numFaces += 1;
        }

        // Check bottom
        if (!cubes.has(buildKey([coordinates[0], coordinates[1], coordinates[2] - 1]))) {
            numFaces += 1;
        }

        // Check left
        if (!cubes.has(buildKey([coordinates[0] - 1, coordinates[1], coordinates[2]]))) {
            numFaces += 1;
        }

        // Check right
        if (!cubes.has(buildKey([coordinates[0] + 1, coordinates[1], coordinates[2]]))) {
            numFaces += 1;
        }

        // Check front
        if (!cubes.has(buildKey([coordinates[0], coordinates[1] + 1, coordinates[2]]))) {
            numFaces += 1;
        }

        // Check back
        if (!cubes.has(buildKey([coordinates[0], coordinates[1] - 1, coordinates[2]]))) {
            numFaces += 1;
        }

    }

    return numFaces;
}


function problem_2(cubes) {

    const coordinates = Array.from(cubes).map(val => val.split("-").map(r => parseInt(r)))
    const limitCube = coordinates.reduce((acc, val) => {

        if (val[0] < acc[0].min)
            acc[0].min = val[0];

        if (val[0] > acc[0].max)
            acc[0].max = val[0];

        if (val[1] < acc[1].min)
            acc[1].min = val[1];

        if (val[1] > acc[1].max)
            acc[1].max = val[1];

        if (val[2] < acc[2].min)
            acc[2].min = val[2];

        if (val[2] > acc[2].max)
            acc[2].max = val[2];

        return acc;
    }, [
        {
            min: Number.MAX_SAFE_INTEGER,
            max: Number.MIN_SAFE_INTEGER
        },
        {
            min: Number.MAX_SAFE_INTEGER,
            max: Number.MIN_SAFE_INTEGER
        },
        {
            min: Number.MAX_SAFE_INTEGER,
            max: Number.MIN_SAFE_INTEGER
        }
    ])

    const coordsStep = [
        [0, 0, -1], // bot
        [0, 0, 1], // top
        [0, -1, 0], // back
        [0, 1, 0], // front
        [1, 0, 0], // left
        [-1, 0, 0] // right
    ];

    limitCube[0].min -= 2;
    limitCube[1].min -= 2;
    limitCube[2].min -= 2;

    limitCube[0].max += 2;
    limitCube[1].max += 2;
    limitCube[2].max += 2;

    const first = [limitCube[0].min, limitCube[1].min, limitCube[2].min];
    let visited = new Set;

    const openSet = [first];
    let count = 0;

    while (openSet.length != 0) {

        const current = openSet.shift();
        const currentKey = buildKey(current);

        if (visited.has(currentKey))
            continue;

        visited.add(currentKey);

        for (let val of coordsStep) {

            let nextCoordinates = [current[0] + val[0], current[1] + val[1], current[2] + val[2]];

            // Limit scope
            if (nextCoordinates[0] < limitCube[0].min || nextCoordinates[0] > limitCube[0].max ||
                nextCoordinates[1] < limitCube[1].min || nextCoordinates[1] > limitCube[1].max ||
                nextCoordinates[2] < limitCube[2].min || nextCoordinates[2] > limitCube[2].max)
                continue;

            const key = buildKey(nextCoordinates);

            if (visited.has(key))
                continue;

            if (cubes.has(key)) {
                count += 1;
                visited.add(nextCoordinates)
                continue;
            }
            
            openSet.push(nextCoordinates);
        };
    }

    return count;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))