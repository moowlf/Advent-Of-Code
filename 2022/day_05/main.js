
const fs = require('fs');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    let stack = {};
    let rules = []
    let onStackParsing = true;

    const hasDigits = function (val) {
        return /\d/.test(val);
    }

    data.split("\n").forEach(val => {

        if (val == "") return;

        if (onStackParsing) {
            const res = val.match(/.{3}\s?/g)

            if (hasDigits(val)) {
                onStackParsing = false;
                return;
            }

            for (let idx = 0; idx < res.length; idx++) {
                const resData = res[idx].replaceAll(" ", "");

                if (resData == "") continue;

                // Data is present
                if (!stack[idx + 1]) {
                    stack[idx + 1] = [];
                }

                // Add element to array
                stack[idx + 1].unshift(res[idx].charAt(1));
            }

        } else {
            // rules
            const values = val.match(/(\d+)/g).map(v => parseInt(v));
            rules.push(values);
        }
    });

    // Parse
    return [stack, rules];
}


//============================================================================
//                                                                      Main >
//============================================================================

const parsedData = readInputFile("./inputs/input.txt");

function run_rules(data, newCrate) {

    const [stack, rules] = data;

    for (let i = 0; i < rules.length; i++) {
        const [amount, from, to] = rules[i];

        const stackToMove = stack[from].slice(stack[from].length - amount);

        if (newCrate == false)
            stackToMove.reverse();

        stack[to].push(...stackToMove);
        stack[from].splice(stack[from].length - amount)
    }
}

function problem_1(parsedData) {
    run_rules(parsedData, false)

    const result = Object.keys(parsedData[0]).reduce((acc, val) => {
        return acc + parsedData[0][val][parsedData[0][val].length - 1];
    }, "");

    return result;
}

function problem_2(parsedData) {
    run_rules(parsedData, true)

    const result = Object.keys(parsedData[0]).reduce((acc, val) => {
        return acc + parsedData[0][val][parsedData[0][val].length - 1];
    }, "");

    return result;
}


console.log("Problem #1: ", problem_1(structuredClone(parsedData)))
console.log("Problem #2: ", problem_2(parsedData))