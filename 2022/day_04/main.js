
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

    const parsedData = data
        .split("\n")
        .map( val => {
            return val.split(",")
        });
    
    return parsedData;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {
    
    return parsedData.reduce( (acc, val) => {

        const [minA, maxA] = val[0].split("-").map( val => parseInt(val));
        const [minB, maxB] = val[1].split("-").map( val => parseInt(val));

        if (
            (minA <= minB && maxA >= maxB) // B in A
            ||
            (minB <= minA && maxB >= maxA) // A in B
        ) return acc + 1;
        
        return acc;
    }, 0);
}

function problem_2(parsedData) {
    
    return parsedData.reduce( (acc, val) => {

        const [minA, maxA] = val[0].split("-").map( val => parseInt(val));
        const [minB, maxB] = val[1].split("-").map( val => parseInt(val));

        if (
            (minB >= minA && minB <= maxA) // minB in A
            ||
            (maxB >= minA && maxB <= maxA) // maxB in A
            ||
            (minA >= minB && minA <= maxB) // minA in B
            ||
            (maxA >= minB && maxA <= maxB) // maxA in B
        ) return acc + 1;
        
        return acc;
    }, 0);
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))