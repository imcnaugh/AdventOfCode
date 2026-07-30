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
