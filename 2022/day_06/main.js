
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
    return data.split("");
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    for (let i = 0; i + 4 < parsedData.length; i++) {

        const a = parsedData[i];
        const b = parsedData[i + 1];
        const c = parsedData[i + 2];
        const d = parsedData[i + 3];

        if (a != b && a != c && a != d
            &&
            b != c && b != d
            &&
            c != d)
            return i + 4;
    }

    return 0;
}

function problem_2(parsedData) {


    const windowChar = {};

    for (let i = 0; i < parsedData.length; i++) {

        if (parsedData[i] in windowChar) {

            const minimum = windowChar[parsedData[i]] + 1;

            [...Object.keys(windowChar)].forEach(val => {
                if (windowChar[val] < minimum) delete windowChar[val];
            });

            windowChar[parsedData[i]] = i;

        } else {
            windowChar[parsedData[i]] = i;
        }

        if (Object.keys(windowChar).length == 14)
            return i + 1;
    }

    return 0;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))