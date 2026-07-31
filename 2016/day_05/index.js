import fs from 'node:fs'
import { createHash } from 'crypto';

const input = fs.readFileSync('input.txt', 'utf8')

const md5Hash = (text) => {
    return createHash('md5').update(text).digest('hex');
};

let match = /^00000/
let output = []
let index = 0;
while(true) {
    const toHash = input + index
    const hashed = md5Hash(toHash)
    if(match.test(hashed)) {
        output.push(hashed.charAt(5))
        if(output.length >= 8){
            break;
        }
    }
    index++
}

const part1 = output.join('')
console.log(`Part 1: ${part1}`)


let matchPart2 = /^00000[0-7]/
let outputPart2 = ['0','0','0','0','0','0','0','0']
let placed = [false,false,false,false,false,false,false,false]
let indexPart2 = 0;
while(true) {
    const toHash = input + indexPart2
    const hashed = md5Hash(toHash)
    if(matchPart2.test(hashed)) {
        const pos = parseInt(hashed.charAt(5))
        if(!placed[pos]){
            outputPart2[pos] = hashed.charAt(6)
            placed[pos] = true
        }
        const all = placed.reduce((acc, c) => acc && c, true)
        if(all){
            break
        }
    }
    indexPart2++
}


const part2 = outputPart2.join('')
console.log(`Part 2: ${part2}`)