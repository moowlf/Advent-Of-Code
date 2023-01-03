
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
    return data.split("\n").map(val => parseInt(val));
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function mod(n, m) {
    return ((n % m) + m) % m;
}

function solve(data) {

    let dec = 0;
    while (dec != data.length) {

        // get next index
        let nextID = data.map(val => val[1]).indexOf(dec);

        const elem = [...data[nextID]];
        dec += 1;

        let nextElement = nextID + elem[0];
        data.splice(nextID, 1);

        nextElement = mod(nextElement, data.length)
        //while (nextElement < 0) { nextElement += data.length; };
        //while (nextElement > data.length) { nextElement -= data.length; };
        data.splice(nextElement, 0, elem);
    }

    return data;
}


function problem_1(data) {

    data = data.map((val, index) => [val, index]);
    decrypted = solve(data).map(val => val[0]);

    let zeroIndex = decrypted.indexOf(0);

    let sum = 0;
    sum += decrypted[(zeroIndex + 1000) % decrypted.length];
    sum += decrypted[(zeroIndex + 2000) % decrypted.length];
    sum += decrypted[(zeroIndex + 3000) % decrypted.length];

    return sum;
}

function problem_2(parsedData) {

    parsedData = parsedData.map((val, index) => [val * 811589153, index]);

    for (let i = 0; i < 10; i++) {
        parsedData = solve(parsedData);
    }

    parsedData = parsedData.map(val => val[0]);
    let zeroIndex = parsedData.indexOf(0);

    let sum = 0;
    sum += parsedData[(zeroIndex + 1000) % parsedData.length];
    sum += parsedData[(zeroIndex + 2000) % parsedData.length];
    sum += parsedData[(zeroIndex + 3000) % parsedData.length];

    return sum;

}


console.log("Problem #1: ", problem_1(parsedData.map(val => val)))
console.log("Problem #2: ", problem_2(parsedData))