import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')
// const input = '  5   10  25'


function isValidTriangle(numbers) {
    let sortedNums = numbers.sort((a, b) => a - b)
    return (sortedNums[0] + sortedNums[1]) > sortedNums[2]
}



const validTriangles = input.split('\n')
    .map(line => line.trim().split(/\s+/).map(Number))
    .filter(numbers => isValidTriangle(numbers))

const totalValidTriangles = validTriangles.length

console.log(`Part 1: ${totalValidTriangles}`)

