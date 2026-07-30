import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

const directions = Object.freeze({
    U: 0,
    L: 1,
    D: 2,
    R: 3,
})

function toDirection(dirStr){
    switch( dirStr ) {
        case 'U':
            return directions.U;
        case 'L':
            return directions.L;
        case 'D':
            return directions.D;
        case 'R':
            return directions.R;
    }
}

const width = 3;
const height = 3;

function moveFromLocation(loc, dir) {
    switch(dir) {
        case directions.U:
            return {x: loc.x, y: Math.max(0, loc.y - 1)};
        case directions.L:
            return {x: Math.max(0, loc.x - 1), y: loc.y};
        case directions.D:
            return {x: loc.x, y: Math.min(loc.y + 1, height - 1)};
        case directions.R:
            return {x: Math.min(loc.x + 1, width - 1), y: loc.y};
    }
}

function getMaxMinYForX(axis) {
    switch(axis) {
        case 0:
        case 4:
            return {max:2, min: 2};
        case 1:
        case 3:
            return {max:3, min: 1};
        case 2:
            return {max:4, min: 0};
    }
}

function moveFromLocationPart2(loc, dir) {
    const { max: maxX, min: minX} = getMaxMinYForX(loc.y)
    const { max: maxY, min: minY } = getMaxMinYForX(loc.x);

    switch(dir) {
        case directions.U:
            return {x: loc.x, y: Math.min(maxY, loc.y + 1)};
        case directions.L:
            return {x: Math.max(minX, loc.x - 1), y: loc.y};
        case directions.D:
            return {x: loc.x, y: Math.max(loc.y - 1, minY)};
        case directions.R:
            return {x: Math.min(loc.x + 1, maxX), y: loc.y};
    }
}

function locToNumPart2({x, y}){
    switch(`${x},${y}`) {
        case '0,2': return '5';
        case '1,3': return '2';
        case '1,2': return '6';
        case '1,1': return 'A';
        case '2,4': return '1';
        case '2,3': return '3';
        case '2,2': return '7';
        case '2,1': return 'B';
        case '2,0': return 'D';
        case '3,3': return '4';
        case '3,2': return '8';
        case '3,1': return 'C';
        case '4,2': return '9';
    }
}

function locToNum(loc) {
    return (loc.y * width + loc.x) + 1;
}

let cur = {x: 1, y: 1}
let nums = []
input.split('\n').forEach(line => {
    line.split('').map(toDirection).forEach(dir => cur = moveFromLocation(cur, dir))
    nums.push(locToNum(cur))
})

console.log(`Part 1: `, nums.join(''))


let curPart2 = {x: 0, y: 2}
let numsPart2 = []

input.split('\n').forEach(line => {
    line.split('').map(toDirection).forEach(dir => curPart2 = moveFromLocationPart2(curPart2, dir))
    numsPart2.push(locToNumPart2(curPart2))
})

console.log(`Part 2: `, numsPart2.join(''))