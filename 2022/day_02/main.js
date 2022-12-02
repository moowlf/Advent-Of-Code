
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
    const plays = [];

    for (let playId = 0; playId < data.length; playId++) {
        plays.push(data[playId].split(" "));
    }

    return plays
}


//============================================================================
//                                                                      Main >
//============================================================================
function convertElfChoice(choice) {
    if (choice == "A") return "X"; // Rock
    else if (choice == "B") return "Y"; // Paper
    else if (choice == "C") return "Z"; // Scissors
    else throw "Invalid elf choice!";
}

function getPlayerLosingHand(elfChoice) {
    if (elfChoice == "A") return "Z"; // Scissors loses to rock
    else if (elfChoice == "B") return "X" //  Rock loses to paper
    else if (elfChoice == "C") return "Y" // Paper loses to scissors
    else throw "Invalid elf hand!";
}

// RPS
function getPlayerWinningHand(elfChoice) {
    if (elfChoice == "A") return "Y"; // Paper wins to rock
    else if (elfChoice == "B") return "Z" //  Scissor wins to paper
    else if (elfChoice == "C") return "X" // Rock wins to scissors
    else throw "Invalid elf hand!";
}

function getPlayPoints(play) {

    let currentScore = 0;

    if (play[1] == "X")
        currentScore += 1; // Rock
    else if (play[1] == "Y")
        currentScore += 2; // Paper
    else if (play[1] == "Z")
        currentScore += 3; // Scissor
    else throw "Invalid user choice";

    const translatedElfChoice = convertElfChoice(play[0]);

    // Draw
    if (translatedElfChoice == play[1]) currentScore += 3;

    // Lose
    else if (translatedElfChoice == "X" && play[1] == "Z" || // Rock - Scissor
        translatedElfChoice == "Y" && play[1] == "X" || // Paper - Rock
        translatedElfChoice == "Z" && play[1] == "Y" // Scissor - Paper
    )
        currentScore += 0;
    // Win
    else
        currentScore += 6;
    
    return currentScore;
}

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {
    let points = parsedData.reduce( (acc, val) => acc + getPlayPoints(val), 0);
    return points;
}

function problem_2(parsedData) {

    let points = parsedData.reduce( (acc, val) => {

        // Adjust play
        if (val[1] == "Y") // Draw
            return acc + getPlayPoints([val[0], convertElfChoice(val[0])]);
        else if (val[1] == "X") // Lose
            return acc + getPlayPoints([val[0], getPlayerLosingHand(val[0])]);
        else if (val[1] == "Z") // Win
            return acc + getPlayPoints([val[0], getPlayerWinningHand(val[0])]);
        else // Invalid option
            throw "Invalid stategy";
    }, 0);

    return points;
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))