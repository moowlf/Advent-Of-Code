
const fs = require('fs');
const Heap = require('heap');

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

    const blueprints = [];
    for (const bp of data) {

        const info = bp.split(/[:\.] ?/);

        const blueprint = {
            id: parseInt(info[0].match(/\d+/)[0]),
            recipes: []
        }

        for (let i = 1; i < info.length - 1; i++) {

            const recipe = info[i].split(" ");
            const recipeObject = {
                target: recipe[1],
                ingredients: []
            }

            for (let j = 4; j < recipe.length; j += 3) {
                recipeObject.ingredients.push([parseInt(recipe[j]), recipe[j + 1]])
            }

            blueprint.recipes.push(recipeObject);
        }

        blueprints.push(blueprint);
    }

    return blueprints;
}


//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");


function translateNameToID(name) {

    if (name == "ore") return 0;
    else if (name == "clay") return 1;
    else if (name == "obsidian") return 2;
    else if (name == "geode") return 3;
    throw "Impossible";
}

function minutesTillHavingResourcesToRobot(recipes, state) {

    let neededTime = 0;

    for (const [qt, ingredient] of recipes.ingredients) {
        const ingredientID = translateNameToID(ingredient);

        if (state.robots[ingredientID] == 0) {
            neededTime = Number.MAX_SAFE_INTEGER;
            break;
        }

        neededTime = Math.max(neededTime, Math.ceil((qt - state.amounts[ingredientID]) / state.robots[ingredientID]));
    }

    return neededTime;
}

function buy(state, maxOres, recipe, bestState, maxTime) {

    const translatedID = translateNameToID(recipe.target);

    // Early exit: No need to build more ore robots if we already producing the max needed at each iteration
    if (state.robots.at(translatedID) >= maxOres.at(translatedID))
        return undefined;

    // Calculate next time t in which we can buy these ingredients
    const next_t = minutesTillHavingResourcesToRobot(recipe, state);

    // If is infinite, we don't know for sure when we can buy
    if (next_t == Number.MAX_SAFE_INTEGER || state.minutes + next_t + 1 > maxTime) return undefined;

    // Create a new state
    const newState = {
        amounts: [...state.amounts],
        robots: [...state.robots],
        minutes: state.minutes + next_t + 1,
    }

    newState.amounts = newState.amounts.map((val, index) => val + state.robots[index] * (next_t + 1));
    newState.robots[translatedID] += 1;

    for (const [qt, ingredient] of recipe.ingredients) {
        const tID = translateNameToID(ingredient);
        newState.amounts[tID] -= qt;
    }

    return newState;
}

function solve_to_minutes(blueprint, state, memory, maxOres, minutes, bestState, depth = 0) {

    const stringy = JSON.stringify(state);

    if (memory.has(stringy)) {
        return memory.get(stringy);
    }

    // Update Best State
    if (state.minutes == minutes) {

        if (state.amounts.at(-1) > bestState.amounts.at(-1)) {
            bestState.minutes = state.minutes;
            bestState.amounts = [...state.amounts];
            bestState.robots = [...state.robots];
        }

        return state.amounts.at(-1);
    }

    // Stop propragating useless children
    const remainingTime = minutes - state.minutes;
    const currentMax = state.amounts.at(-1) + remainingTime * state.robots.at(-1);
    const bestScenario = remainingTime * (remainingTime + 1) / 2;
    if (currentMax + bestScenario <= bestState.amounts.at(-1)) {
        return -1;
    }

    // Create children of the current state
    let possibleAmount = 0;
    const geode = buy(state, maxOres, blueprint.recipes[3], bestState, minutes);
    if (geode != undefined)
        possibleAmount = Math.max(possibleAmount, solve_to_minutes(blueprint, geode, memory, maxOres, minutes, bestState, depth + 1));

    const obsidian = buy(state, maxOres, blueprint.recipes[2], bestState, minutes);
    if (obsidian != undefined)
        possibleAmount = Math.max(possibleAmount, solve_to_minutes(blueprint, obsidian, memory, maxOres, minutes, bestState, depth + 1));

    const clay = buy(state, maxOres, blueprint.recipes[1], bestState, minutes);
    if (clay != undefined)
        possibleAmount = Math.max(possibleAmount, solve_to_minutes(blueprint, clay, memory, maxOres, minutes, bestState, depth + 1));

    const ore = buy(state, maxOres, blueprint.recipes[0], bestState, minutes);
    if (ore != undefined)
        possibleAmount = Math.max(possibleAmount, solve_to_minutes(blueprint, ore, memory, maxOres, minutes, bestState, depth + 1));

    if (state.minutes + 1 <= minutes) {

        const updatedAmounts = state.amounts.map((val, index) => val + state.robots[index]);
        const timeOnly = {
            "amounts": updatedAmounts,
            "robots": [...state.robots],
            "minutes": state.minutes + 1,
        }

        possibleAmount = Math.max(possibleAmount, solve_to_minutes(blueprint, timeOnly, memory, maxOres, minutes, bestState, depth + 1));
    }

    memory.set(stringy, possibleAmount);
    return memory.get(stringy);
}


function solve(parsedData, minutes, part1) {

    const geodes = [];

    parsedData.forEach((blueprint, index) => {

        // Get max of ores needed at any minute
        const max_ores = [0, 0, 0, Number.MAX_SAFE_INTEGER];
        for (const recipe of blueprint.recipes) {
            for (const [qt, id] of recipe.ingredients) {
                const translatedID = translateNameToID(id);
                max_ores[translatedID] = Math.max(max_ores[translatedID], qt);
            }
        }

        const beginnerState = {
            "amounts": [0, 0, 0, 0],
            "robots": [1, 0, 0, 0],
            "minutes": 0,
        };

        const bestState = {
            minutes: Number.MAX_SAFE_INTEGER,
            amounts: [0, 0, 0, Number.MIN_SAFE_INTEGER],
            robots: 0
        }

        const best = solve_to_minutes(blueprint, beginnerState, new Map, max_ores, minutes, bestState);

        if (part1)
            geodes.push((index + 1) * best);
        else
            geodes.push(best);
    });

    return geodes;
}


function problem_1(parsedData) {
    const geodes = solve(parsedData, 24, true);
    return geodes.reduce((acc, val) => val + acc, 0);
}

function problem_2(parsedData) {
    const geodes = solve(parsedData.filter((val, index) => index < 3), 32, false);
    return geodes.reduce((acc, val) => val * acc, 1);
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData,))