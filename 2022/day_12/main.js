
const fs = require('fs');
const Graph = require('node-dijkstra');
const path = require('path');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    return data.split("\n").map(val => val.split(""));
}


//============================================================================
//                                                                      Main >
//============================================================================
let parsedData = readInputFile("./inputs/input.txt");
let parsedData2 = readInputFile("./inputs/input.txt");


function build_graph(data) {

    const route = new Graph;

    for (let y = 0; y < data.length; y++) {
        for (let x = 0; x < data[y].length; x++) {

            const neighbours = {};
            const current_letter_code = data[y][x].charCodeAt(0);

            // Check if left neighbour is valid
            if (x > 1) {
                const left_lc = data[y][x - 1].charCodeAt(0);
                if (current_letter_code >= left_lc || current_letter_code + 1 == left_lc) {
                    neighbours[`Y${y}X${x - 1}`] = 1;
                }
            }

            // Check if right neighbour is valid
            if (x < data[0].length - 1) {
                const right_lc = data[y][x + 1].charCodeAt(0);
                if (current_letter_code >= right_lc || current_letter_code + 1 == right_lc) {
                    neighbours[`Y${y}X${x + 1}`] = 1;
                }
            }

            // Check if top neighbour is valid
            if (y > 0) {
                const top_lc = data[y - 1][x].charCodeAt(0);
                if (current_letter_code >= top_lc || current_letter_code + 1 == top_lc) {
                    neighbours[`Y${y - 1}X${x}`] = 1;
                }
            }

            // Check if bottom neighbour is valid
            if (y < data.length - 1) {
                const bottom_lc = data[y + 1][x].charCodeAt(0);
                if (current_letter_code >= bottom_lc || current_letter_code + 1 == bottom_lc) {
                    neighbours[`Y${y + 1}X${x}`] = 1;
                }
            }

            route.addNode(`Y${y}X${x}`, neighbours);
        }
    }

    return route;
}

function problem_1(data) {

    for (let y = 0; y < data.length; y++) {
        for (let x = 0; x < data[y].length; x++) {

            if (data[y][x] == "S") {
                start = [y, x];
                data[y][x] = "a";
            }
            else if (data[y][x] == "E") {
                end = [y, x];
                data[y][x] = "z";
            }
        }
    }

    const graph = build_graph(data);
    const r = graph.path(`Y${start[0]}X${start[1]}`, `Y${end[0]}X${end[1]}`);

    return r.length - 1;
}

function problem_2(parsedData) {

    let end = undefined;

    const a_coordinates = [];
    for (let y = 0; y < parsedData.length; y++) {
        for (let x = 0; x < parsedData[y].length; x++) {

            if (parsedData[y][x] == "a") {
                a_coordinates.push([y, x]);
            }
            else if (parsedData[y][x] == "E") {
                end = [y, x];
                parsedData[y][x] = "z";
            } else if (parsedData[y][x] == "S") {
                a_coordinates.push([y, x]);
                parsedData[y][x] = "a";
            }
        }
    }

    const graph = build_graph(parsedData)

    let min = Number.MAX_SAFE_INTEGER;
    for (let i = 0; i < a_coordinates.length; i++) {
        const p = graph.path(`Y${a_coordinates[i][0]}X${a_coordinates[i][1]}`, `Y${end[0]}X${end[1]}`);
        if (p != undefined && p.length - 1 < min) {
            min = p.length - 1;
        }
    };

    return min;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData2))