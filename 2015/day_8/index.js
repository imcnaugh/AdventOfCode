import fs from 'fs';

let input = fs.readFileSync('input.txt', 'utf8');
// let input = fs.readFileSync('test.txt', 'utf8');

// let output = input.split('\n').reduce((acc, line) => {
//     let originalLength = line.length;
//     line = line.slice(1, -1);
//
//     line = line.replace(/\\x[\da-f]{2}/g, "a");
//     line = line.replace(/\\"/g, "a");
//     line = line.replace(/\\\\/g, 'a');
//
//     return acc + originalLength - line.length;
// }, 0)

let output = input.split('\n').reduce((acc, line) => {
    let originalLength = line.length;
    line = line.replace(/\\/g, "\\\\");
    line = line.replace(/"/g, "\\\"");

    // console.log(line);

    return acc + line.length - originalLength + 2;
}, 0);

console.log(output);