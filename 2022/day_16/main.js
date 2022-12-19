
const fs = require('fs');
const Graph = require('node-dijkstra')
const Heap = require('heap')

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });


    data = data.split("\n").map(val => {

        const [initialValve, ...valvs] = val.match(/[A-Z]{2}/g);
        const [flowRate] = val.match(/\d+/g);

        return {
            origin: initialValve,
            destination: valvs,
            flow: parseInt(flowRate)
        }
    }).reduce((acc, val) => {

        acc[val.origin] = {
            destination: val.destination,
            flow: val.flow
        }

        return acc;
    }, {});

    // Parse
    return data
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function calculatePaths(data) {

    // Build graph
    const graph = new Graph;
    for (let key of Object.keys(data)) {
        const obj = {};
        for (let neighboor of data[key].destination) {
            obj[neighboor] = 1;
        }
        graph.addNode(key, obj);
    }

    // calculate distances
    const distances = {};

    for (let from of Object.keys(data)) {
        for (let to of Object.keys(data)) {
            if (from == to) distances[`${from}${to}`] = 0;
            else distances[`${from}${to}`] = graph.path(from, to);
        }
    }

    return distances;

}

function solve(data, maxTime) {

    // Pre compute all the paths between nodes
    const paths = calculatePaths(data);

    // Get all nodes in which there's a valve to open
    const valves = Object.keys(data).filter(val => data[val].flow != 0);

    // Create an openset of possibilites
    const openSet = new Heap;
    openSet.push({ name: "AA", duration: 0, pressure: 0, used: "" });

    // Search
    const possibleSolutions = [];

    while (!openSet.empty()) {

        const current_state = openSet.pop();

        possibleSolutions.push(current_state)

        // Get valves yet to activate
        const to_activate = valves.filter(val => current_state.used.indexOf(val) == -1);

        // Set focus to unused paths
        for (const valve of to_activate) {
            const distanceToValve = paths[`${current_state.name}${valve}`].length - 1;

            // No need to go there, since going there and activate the valve would excede our
            //  30 seconds.
            if (current_state.duration + distanceToValve + 1 > maxTime) continue;

            const newDuration = current_state.duration + distanceToValve + 1
            const newUsed = `${current_state.used}-${valve}`;
            const newPressure = current_state.pressure + data[valve].flow * (maxTime - newDuration);

            openSet.push({ name: valve, duration: newDuration, pressure: newPressure, used: newUsed });
        }
    }

    possibleSolutions.sort((a, b) => b.pressure - a.pressure);
    return possibleSolutions
}

function solve2(data) {

    const max_time = 26;
    const all_paths = solve(data, max_time);

    let best_pressure = 0
    let best_so_far = [-1, -1];

    for (let i = 0; i < all_paths.length; i++) {

        const current_path = all_paths[i].used.split('-').slice(1);
        const current_path_pressure = all_paths[i].pressure;

        for (let j = 0; j < all_paths.length; j++) {

            if (i == j) continue;
            const comparative_path_pressure = all_paths[j].pressure;

            if (current_path_pressure + comparative_path_pressure < best_pressure) break;

            const comparative_path = all_paths[j].used.split("-").slice(1);
            const repeatedElements = !comparative_path.some(val => current_path.includes(val));

            if (repeatedElements && best_pressure < current_path_pressure + comparative_path_pressure) {
                best_pressure = current_path_pressure + comparative_path_pressure;
                best_so_far = [i, j];
            }
        }
    }

    return best_pressure
}

function problem_1(parsedData) {
    return solve(parsedData, 30);
}

function problem_2(parsedData) {
    return solve2(parsedData);
}


console.log("Problem #1: ", problem_1(parsedData)[0].pressure)
console.log("Problem #2: ", problem_2(parsedData))
