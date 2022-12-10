
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
    return data.split("\n").map(val => val.split(" "));
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    const shouldMultiply = function (index) {
        return index % 40 == 20;
    }

    let x = 1;
    let sumSignal = 0;
    let currentCycle = 1;

    for (let i = 0; i < parsedData.length; i++) {

        const [op, value] = parsedData[i];

        if (op == "noop") {
            currentCycle++;
        } else {
            currentCycle++;

            if (shouldMultiply(currentCycle)) {
                sumSignal += currentCycle * x;
            }

            x += parseInt(value);
            currentCycle++;
        }

        if (shouldMultiply(currentCycle)) {
            sumSignal += currentCycle * x;
        }
    }


    return sumSignal;
}

function problem_2(parsedData) {

    let crt = [
        Array.from({ length: 40 }, val => "."),
        Array.from({ length: 40 }, val => "."),
        Array.from({ length: 40 }, val => "."),
        Array.from({ length: 40 }, val => "."),
        Array.from({ length: 40 }, val => "."),
        Array.from({ length: 40 }, val => "."),
    ]

    const draw = function (currentCycle, spritePosition) {
        const line = Math.floor(currentCycle / 40);
        const pixel = currentCycle % 40;

        if (pixel >= spritePosition - 1 && pixel <= spritePosition + 1) {
            crt[line][pixel] = "#";
        }

    };


    let x = 1;
    let currentCycle = 0;

    for (let i = 0; i < parsedData.length; i++) {

        const [op, value] = parsedData[i];
        draw(currentCycle, x);

        if (op == "noop") {
            currentCycle++;
        }
        else {
            currentCycle++;
            draw(currentCycle, x);

            x += parseInt(value);
            currentCycle++;
        }

    }

    for (let i = 0; i < crt.length; i++) {
        console.log(crt[i].join(""))
    }


    return 0;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))