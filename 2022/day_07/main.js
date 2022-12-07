
const fs = require('fs');

//============================================================================
//                                                         Read & Parse file >
//============================================================================

class FileSystemPath {

    constructor(path) {
        this.path = path;
        this.files = new Set();
        this.dirs = new Set();
        this.size = 0;
    }
}

function readInputFile(fileLocation) {

    let data = fs.readFileSync(fileLocation, 'utf-8', (err, data) => {
        if (err) throw "Failed to read file!";
        return data;
    });

    // Parse
    data = data.split("\n");
    const fileSystem = [
        new FileSystemPath("/")
    ];
    let current = "/";

    const getNode = function (path) {
        return fileSystem.filter(val => val.path == path);
    };

    for (let i = 1; i < data.length; i++) {
        // Check if command
        if (data[i].startsWith("$")) {

            // Deal with cd
            if (data[i].includes("cd")) {
                const dir = data[i].match(/\$\scd\s(.+)/)[1];
                const [parent] = getNode(current);
                
                if (dir == "/") {
                    current = "/";
                }
                else if (dir != "..") {
                    current = `${current}${dir}/`;
                    const newPath = `${current}`;
                    parent.dirs.add(dir);

                    const [ children ] = getNode(newPath);
                    if (children == null) {
                        fileSystem.push(new FileSystemPath(newPath))
                    }

                }
                else {
                    current = `${current.split("/").slice(0, -2).join("/")}/`
                    continue;
                }

            } // Deal with ls 
            else if (data[i].includes("ls")) {
                // we're doing nothing, since we will be reading next
            }
            else {
                throw "Invalid command"
            }
            continue;
        }

        // Check result of ls
        const [parent] = getNode(current);
        if (data[i].includes("dir")) {
            parent.dirs.add(data[i].split(" ")[1]);
        }
        else {
            const match = data[i].split(" ");
            parent.files.add([match[1], match[0]]);
            parent.size += parseInt(match[0]);
        }
    }

    return fileSystem;
}

//============================================================================
//                                                                      Main >
//============================================================================

let parsedData = readInputFile("./inputs/input.txt");

function problem_1(parsedData) {

    const sizes = {};

    const recursiveCount = function (cwd) {

        if (cwd in Object.keys(sizes)) {
            return sizes[cwd];
        }

        const [currentNode] = parsedData.filter(val => val.path == cwd);

        let childrenSize = 0;
        for (const dirElement of currentNode.dirs)
            childrenSize += recursiveCount(`${currentNode.path}${dirElement}/`);

        sizes[cwd] = childrenSize + currentNode.size;
        return childrenSize + currentNode.size;
    }

    return parsedData.reduce((acc, val) => {

        const size = recursiveCount(val.path);
        if (size <= 100000)
            return acc + size;
        return acc;
    }, 0);
}

function problem_2(parsedData) {

    const sizes = {};
    const recursiveCount = function (cwd) {
        if (cwd in Object.keys(sizes)) {
            return sizes[cwd];
        }

        const [currentNode] = parsedData.filter(val => val.path == cwd);
        let childrenSize = 0;
        for (const dirElement of currentNode.dirs)
            childrenSize += recursiveCount(`${currentNode.path}${dirElement}/`);

        sizes[cwd] = childrenSize + currentNode.size;
        return childrenSize + currentNode.size;
    }
    
    recursiveCount("/");

    const arr = Object.entries(sizes);
    arr.sort( (a, b) => b[1] - a[1]);
    
    const spaceNeeded = 30_000_000 - (70_000_000 - sizes["/"]);

    const possibilities = arr.filter( val => val[1] >= spaceNeeded);

    return possibilities.pop()[1]
}


console.log("Problem #1: ", problem_1(parsedData))
console.log("Problem #2: ", problem_2(parsedData))