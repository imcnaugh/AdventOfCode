import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')
// const input = "abcd[bddb]xyyx"

const abbaRx = /(.)(?!\1)(.)\2\1/
const bracketRx = /\[.*?]/g

function isABBASupported(line) {
    const bracketMatch = line.match(bracketRx);

    for(const match of bracketMatch){
        if(abbaRx.test(match)){
            return false
        }
    }

    return abbaRx.test(line)
}

const part1 = input.split('\n').filter(line => {
    return isABBASupported(line)
}).length

console.log(`Part 1: ${part1}`)