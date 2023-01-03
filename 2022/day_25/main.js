
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
    return data.split("\n").map(val => val.split(""));
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function convertToDecimal(arr) {

    const weirdAlphToDec = val => {
        if (val == '2') return 2;
        else if (val == '1') return 1;
        else if (val == '0') return 0;
        else if (val == '-') return -1;
        else return -2;
    }

    let dec = 0;
    for (let i = 0; i < arr.length; i++) {
        const val = weirdAlphToDec(arr[i]);
        dec += val * Math.pow(5, arr.length - i - 1);
    }
    return dec;
}

function convertToWeirdBase5(val) {

    const decToWeirdAlph = val => {
        if (val == 4) return '2';
        else if (val == 3) return '1';
        else if (val == 2) return '0';
        else if (val == 1) return '-';
        else return '=';
    }

    let b5 = [];
    let borrow = 0;
    while (true) {
        const res = (val % 5) + borrow;

        if (res > 2) {
            b5.unshift(decToWeirdAlph(res - 3))
            borrow = 1;
        }
        else {
            b5.unshift(decToWeirdAlph(res + 2));
            borrow = 0;
        }

        val = Math.floor(val / 5);

        if (val == 0 && borrow == 0)
            break;
    }

    return b5.join("");
}

function problem_1(parsedData) {
    const sum = parsedData.reduce((acc, val) => acc + convertToDecimal(val), 0);
    return convertToWeirdBase5(sum);
}

console.log("Problem #1: ", problem_1(parsedData))