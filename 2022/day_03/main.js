
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

    const parsedData = data.split("\n").map( val => {

        const a = val.substring(0, val.length / 2);
        const b = val.substring(val.length / 2);
        return [a, b]
    });

    return parsedData
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function getPriority(ch) {

    // Check if letter is uppercase
    if (ch.toUpperCase() == ch) {
        return ch.charCodeAt(0) - "A".charCodeAt(0) + 27;
    }

    // Lowercase
    return ch.charCodeAt(0) - "a".charCodeAt(0) + 1;
}

function problem_1(parsedData) {

    const priorityValue = parsedData.reduce( (acc, val) => {

        const firstCompartment = new Set(val[0])
        for (let idx = 0; idx < val[1].length; idx++) {
            if (firstCompartment.has(val[1].charAt(idx))) {
                return acc + getPriority(val[1].charAt(idx));
            }
        }
    }, 0);

    return priorityValue;
}

function problem_2(parsedData) {

    let priority = 0;
    for (let i = 0; i < parsedData.length; i+= 3) {

        const arr_a = Array.from(new Set(parsedData[i][0].concat(parsedData[i][1]).split("")));

        const arr_b = parsedData[i+1][0].concat(parsedData[i+1][1]).split("");
        const b = new Set(arr_b);

        const arr_c = parsedData[i+2][0].concat(parsedData[i+2][1]).split("");
        const c = new Set(arr_c);

        let intersect = arr_a.filter( v => b.has(v) && c.has(v));
        priority += getPriority(intersect[0])
    }

    return priority;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))