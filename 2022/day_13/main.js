
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

    data = data.split("\n")

    const parsedData = []
    for (let i = 0; i < data.length; i += 3) {

        parsedData.push({})
        parsedData[parsedData.length - 1]["left"] = eval(data[i]);
        parsedData[parsedData.length - 1]["right"] = eval(data[i + 1]);
    }

    return parsedData
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");


function compare(val1, val2) {
    /*
         1 -> val1 < val2
         0 -> val1 == val2
        -1 -> val1 > val2
    */

    // First Rule
    if (Number.isInteger(val1) && Number.isInteger(val2)) {
        if (val1 < val2) return 1;
        else if (val1 == val2) return 0;
        else return -1;
    }

    // Second Rule
    if (Array.isArray(val1) && Array.isArray(val2)) {

        let i = 0;
        while (true) {

            if (i >= val1.length && i >= val2.length)
                return 0;
            else if (i >= val1.length && i < val2.length)
                return 1;
            else if (i < val1.length && i >= val2.length)
                return -1;

            let res = compare(val1[i], val2[i]);

            if (res != 0) return res;
            i++;
        }
    }

    // Third Rule
    if (Array.isArray(val1) && Number.isInteger(val2)) {
        return compare(val1, [val2]);
    }
    else if (Number.isInteger(val1) && Array.isArray(val2)) {
        return compare([val1], val2);
    }

    console.log("Something fishy happened");

}

function problem_1(parsedData) {

    let sum = 0;
    for (let i = 0; i < parsedData.length; i++) {

        let ordered = compare(parsedData[i].left, parsedData[i].right);
        if (ordered == 1) {
            sum += (i + 1);
        }
    }
    return sum;
}

function problem_2(parsedData) {

    // Create array with all packets
    let all_packages = []
    parsedData.forEach(element => {
        all_packages.push(element.left)
        all_packages.push(element.right)
    });
    all_packages.push([[2]], [[6]])

    // Sort
    all_packages.sort((a, b) => compare(b, a));

    all_packages = all_packages.map(val => JSON.stringify(val))

    const firstDivider = all_packages.indexOf("[[2]]")
    const secondDivider = all_packages.indexOf("[[6]]");

    return (firstDivider + 1) * (secondDivider + 1);
}

console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))