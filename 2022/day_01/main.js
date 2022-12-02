
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

    let amountOfElfBagsParsed = 0;
    let parsedData = [[]];
    for (let i = 0; i < data.length; i++) {
        
        if (data[i] == "") {
            amountOfElfBagsParsed +=1;
            parsedData.push([]);
            continue;
        }

        parsedData[amountOfElfBagsParsed].push(parseInt(data[i]));
    }

    return parsedData
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {
    
    let d = parsedData.map(val => {
        return val.reduce( (acc, val) => acc + val, 0)
    });
    d.sort( (a,b) => b - a);
    return d[0];
}

function problem_2(parsedData) {

    let d = parsedData.map(val => {
        return val.reduce( (acc, val) => acc + val, 0)
    });
    d.sort( (a,b) => b - a);
    return d[0] + d[1] + d[2]
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))