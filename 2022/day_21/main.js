
const fs = require('fs');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

function convertToObj(val) {
    return {
        dividend: val,
        divisor: 1
    };
}

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    const parsedData = {};
    data.split("\n").forEach(val => {

        const vals = val.split(" ");

        const result = {}

        if (vals.length < 3) {
            result["value"] = convertToObj(vals[1]);
        } else {
            result["op"] = vals.slice(1);
        }

        parsedData[vals[0].substring(0, vals[0].length - 1)] = result;
    });
    return parsedData;
}


//============================================================================
//                                                                      Main >
//============================================================================


function add(x, y) {
    return {
        dividend: x.dividend * y.divisor + y.dividend * x.divisor,
        divisor: x.divisor * y.divisor
    };
}

function sub(x, y) {
    return {
        dividend: x.dividend * y.divisor - y.dividend * x.divisor,
        divisor: x.divisor * y.divisor
    };
}

function div(x, y) {
    return {
        dividend: x.dividend * y.divisor,
        divisor: x.divisor * y.dividend
    };
}

function mul(x, y) {
    return {
        dividend: x.dividend * y.dividend,
        divisor: x.divisor * y.divisor
    };
}



function solve(name, data) {

    if (data[name].hasOwnProperty("value")) return data[name].value;

    let val = 0;
    switch (data[name].op[1]) {
        case "+":
            val = add(solve(data[name].op[0], data), solve(data[name].op[2], data))
            break;
        case "-":
            val = sub(solve(data[name].op[0], data), solve(data[name].op[2], data))
            break;
        case "/":
            val = div(solve(data[name].op[0], data), solve(data[name].op[2], data))
            break;
        case "*":
            val = mul(solve(data[name].op[0], data), solve(data[name].op[2], data))
            break;
    }

    return val;
}

function hasChild(name, data, val) {

    if (name == val) return true;

    if (data[name].hasOwnProperty("op"))
        return hasChild(data[name].op[0], data, val) || hasChild(data[name].op[2], data, val);

    return false;
}

function solveFor(name, data, value) {


    if (name == "humn") { data[name].value = value; return value; }
    if (data[name].hasOwnProperty("value")) return data[name].value;

    const [name1, op, name2] = data[name].op;
    const leftChild = hasChild(name1, data, "humn");

    switch (op) {
        case "*":
            if (leftChild) {
                return solveFor(name1, data, div(value, solve(name2, data)));
            } else {
                return solveFor(name2, data, div(value, solve(name1, data)));
            }
        case "/":
            if (leftChild) {
                const rightChild = solve(name2, data);
                return solveFor(name1, data, mul(value, rightChild));
            } else {
                const leftChild = solve(name1, data);
                return solveFor(name2, data, div(leftChild, value));
            }
        case "-":

            if (leftChild) {
                return solveFor(name1, data, add(value, solve(name2, data)));
            } else {
                return solveFor(name2, data, add(mul(value, convertToObj(-1)), solve(name1, data)));
            }

        case "+":

            if (leftChild) {
                return solveFor(name1, data, sub(value, solve(name2, data)));
            } else {
                return solveFor(name2, data, sub(value, solve(name1, data)));
            }
    }

}

function solve2(data) {

    const [name1, op, name2] = data["root"].op;

    const rootIn1 = hasChild(name1, data, "humn");

    if (rootIn1) {
        const p =  solve(name2, data);
        return solveFor(name1, data, p);
    } else {
        return solveFor(name2, data, solve(name1, data));
    }
}

function problem_1() {
    let parsedData = readInputFile("./inputs/input.txt");
    const d = solve("root", parsedData);
    return d.dividend / d.divisor
}

function problem_2() {
    let parsedData = readInputFile("./inputs/input.txt");
    const d = solve2(parsedData);
    return d.dividend / d.divisor
}

console.log("Problem #1: ", problem_1())
console.log("Problem #2: ", problem_2())
