import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

const height = 6;
const width = 50;

let displayPixels = Array.from({ length: width }, () => Array(height).fill(false));

const rect = (a, b) => ({type:"rect", a, b})
const rotRow = (y, shift) => ({type: "rotRow", y, shift})
const rotCol = (x, shift) => ({type: "rotCol", x, shift})

const rectRx = /(\d+)x(\d+)/
const rotRowRx = /y=(\d+) by (\d+)/
const rotColRx = /x=(\d+) by (\d+)/

function executeRect(command) {
    for(let r = 0; r < command.a; r++){
        for(let c = 0; c < command.b; c++){
            displayPixels[r][c] = true
        }
    }
}

function executeRotRow(command) {
    let newRow = Array(width).fill(false)
    for(let i = 0; i < width; i++){
        newRow[(i+command.shift) % width] = displayPixels[i][command.y]
    }
    for(let i = 0; i < width; i++){
        displayPixels[i][command.y] = newRow[i]
    }
}

function executeRotCol(command){
    let newCol = Array(height).fill(false)
    for(let i = 0; i < height; i++){
        newCol[(i+command.shift) % height] = displayPixels[command.x][i]
    }
    for(let i = 0; i < height; i++){
        displayPixels[command.x][i] = newCol[i]
    }
}

input.split('\n').map(line => {
    if(line.startsWith("rect")){
        const matches = line.match(rectRx)
        return rect(parseInt(matches[1]), parseInt(matches[2]))
    } else if(line.startsWith("rotate row")){
        const matches = line.match(rotRowRx)
        return rotRow(parseInt(matches[1]), parseInt(matches[2]))
    } else if(line.startsWith("rotate column")){
        const matches = line.match(rotColRx)
        return rotCol(parseInt(matches[1]), parseInt(matches[2]))
    }
    return null
}).forEach(command => {
    switch(command.type){
        case "rect":
            executeRect(command)
            break
        case "rotRow":
            executeRotRow(command)
            break
        case "rotCol":
            executeRotCol(command)
            break
    }
})

const part1 = displayPixels.map(row => {
    return row.filter(c => c === true).length
}).reduce((acc, c) => acc + c, 0)

console.log(`Part 1: ${part1}`)

for(let c = 0; c < height; c++){
    let chars = []
    for(let r = 0; r < width; r++) {
        const char = displayPixels[r][c] ? "X" : " "
        chars.push(char)
    }
    console.log(chars.join(''))
}