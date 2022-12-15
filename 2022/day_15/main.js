
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
    data = data.split("\n").map(val => {
        const matches = val.match(/(-?\d+)/g);
        return {
            "sensor": {
                "x": parseInt(matches[0]),
                "y": parseInt(matches[1])
            },
            "beacon": {
                "x": parseInt(matches[2]),
                "y": parseInt(matches[3])
            },
        };
    });

    return data;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");
let parsedData2 = readInputFile("./inputs/input.txt");

function manhattan(pos1, pos2) {
    return Math.abs(pos2.x - pos1.x) + Math.abs(pos2.y - pos1.y);
}

function problem_1(parsedData, targetY) {

    const beacon_impossibilities = new Set;

    for (let i = 0; i < parsedData.length; i++) {
        const manhattan_distance = manhattan(parsedData[i].sensor, parsedData[i].beacon);
        const current = parsedData[i].sensor;

        if (current.y < targetY && current.y + manhattan_distance < targetY) {
            continue;
        }

        if (current.y > targetY && current.y - manhattan_distance > targetY) {
            continue;
        }

        beacon_impossibilities.add(current.x);

        // Add new impossibilities
        let diffY = 0;

        if (current.y < targetY) {
            diffY = Math.abs(manhattan_distance - Math.abs(targetY - current.y));
        } else if (current.y > targetY) {
            diffY = Math.abs(manhattan_distance - Math.abs(targetY - current.y));
        }

        for (let i = 0; i < diffY; i++) {
            beacon_impossibilities.add(current.x + (i + 1));
            beacon_impossibilities.add(current.x - (i + 1));
        }
    }

    parsedData.forEach(val => {

        if (val.beacon.y == targetY && beacon_impossibilities.has(val.beacon.x))
            beacon_impossibilities.delete(val.beacon.x);
    });

    return beacon_impossibilities.size;
}

function problem_2(parsedData) {

    let outliers = new Set;

    const distances = [];
    for (let local of parsedData) {
        distances.push([local.sensor, manhattan(local.sensor, local.beacon)]);
    }


    for (let i = 0; i < parsedData.length; i++) {

        console.log(i)
        const sensor = parsedData[i].sensor;
        const beacon = parsedData[i].beacon;
        const distance = manhattan(sensor, beacon);

        let current_borders = [[sensor.x, sensor.y + distance + 1], [sensor.x, sensor.y - distance - 1]];
        let stepX = 1;

        // Add bottom side
        while (true) {

            const new_y = sensor.y + distance + 1 - stepX;
            const right = [sensor.x + stepX, new_y];
            const left = [sensor.x - stepX, new_y];
            current_borders.push(right, left);

            if (right[1] <= sensor.y)
                break;

            stepX += 1;
        }

        // Add top side
        stepX = 0;
        while (true) {

            const new_y = sensor.y - distance - 1 + stepX;

            const right = [sensor.x + stepX, new_y];
            const left = [sensor.x - stepX, new_y];
            current_borders.push(right, left);

            if (right[1] == sensor.y)
                break;

            stepX += 1;
        }

        current_borders = current_borders.filter(val => val[0] >= 0 && val[0] <= 4000000 && val[1] >= 0 && val[1] <= 4000000)

        for (let border of current_borders) {

            const valid = distances.every(val => {
                return manhattan({ x: border[0], y: border[1] }, val[0]) > val[1];
            })

            if (valid){
                outliers.add(`${border[0]}.${border[1]}`);
                console.log("added", border)
            }
        }
    }

    /*
        for (let j = 0; j < parsedData.length; j++) {
    
            const distance = manhattan(parsedData[j].sensor, parsedData[j].beacon);
            const possible_keys = Array.from(outliers.keys());
    
            for (let key of possible_keys) {
                const coords = key.split(".").map(val => parseInt(val));
    
                if (manhattan({ x: coords[0], y: coords[1] }, parsedData[j].sensor) <= distance) {
                    outliers.delete(key);
                }
            }
        }*/

    let [beacon] = outliers;
    console.log(beacon)
    beacon = beacon.split(".").map(val => parseInt(val));

    return beacon[0] * 4000000 + beacon[1];
}


// console.log("Problem #1: ", problem_1(parsedData, 2000000))
console.log("Problem #2: ", problem_2(parsedData2))