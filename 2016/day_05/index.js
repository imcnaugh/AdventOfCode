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
