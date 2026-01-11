import fs from 'fs';
const input = fs.readFileSync('./input.txt', 'utf-8');

let lights = new Array(1000000).fill(false);
const sumOfLights = () => lights.reduce((acc, el) => acc + el, 0);

class Grid {
    constructor(x1, y1, x2, y2) {
        this.x1 = Math.min(x1, x2);
        this.y1 = Math.min(y1, y2);
        this.x2 = Math.max(x1, x2);
        this.y2 = Math.max(y1, y2);
    }
}

const instruction = {
    ON: 0,
    OFF: 1,
    TOGGLE: 2,
}
Object.freeze(instruction);

const toggle = (grid, inst) => {
    for (let x = grid.x1; x <= grid.x2; x++) {
        for (let y = grid.y1; y <= grid.y2; y++) {
            let index = x * 1000 + y;
            if (inst === instruction.ON) {
                lights[index] = lights[index] + 1;
            } else if (inst === instruction.OFF) {
                lights[index] = Math.max(lights[index] -1, 0);
            } else if (inst === instruction.TOGGLE) {
                lights[index] = lights[index] + 2;
            }
        }
    }
}

input.split('\n').forEach((line) => {
    const match = line.match(/(\d+),(\d+) through (\d+),(\d+)/);
    let grid = new Grid(match[1], match[2], match[3], match[4]);
    if (line.startsWith('turn on')) {
        toggle(grid, instruction.ON);
    } else if (line.startsWith('turn off')) {
        toggle(grid, instruction.OFF);
    } else if (line.startsWith('toggle')) {
        toggle(grid, instruction.TOGGLE);
    }
})

console.log(sumOfLights());