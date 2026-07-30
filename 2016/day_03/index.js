import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

function isValidTriangle(numbers) {
    let sortedNums = numbers.sort((a, b) => a - b)
    return (sortedNums[0] + sortedNums[1]) > sortedNums[2]
}

const validTriangles = input.split('\n')
    .map(line => line.trim().split(/\s+/).map(Number))
    .filter(numbers => isValidTriangle(numbers))

const totalValidTriangles = validTriangles.length

console.log(`Part 1: ${totalValidTriangles}`)

const d2 = input.split('\n').map(line => line.trim().split(/\s+/).map(Number))
const numbers = []
for (let i = 0; i < 3; i++){
    for(let j = 0; j < d2.length; j++){
        numbers.push(d2[j][i])
    }
}

let part2Valid = 0;
for(let i = 0; i < numbers.length; i += 3){
    if(isValidTriangle(numbers.slice(i, i + 3)))
        part2Valid++
}

console.log(`Part 2: ${part2Valid}`)