import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')
// const input = "aba[bab]xyz\n" +
//     // "xyx[xyx]xyx\n" +
//     // "aaa[kek]eke\n" +
//     "zazbz[bzb]cdb\n" +
//     "bzb[zazbz]cdb"

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

const part1 = input.split('\n').filter(line => isABBASupported(line)).length

console.log(`Part 1: ${part1}`)


const abaRx = /(?=((.)(?!\2).\2))/g

function isSSLSupported(line) {
    const lineInBrackets = line.match(bracketRx).join(' ')
    const lineNoBrackets = line.replace(bracketRx, '   ')

    const bracketMatches = [...lineInBrackets.matchAll(abaRx)].map(m => m[1]);

    for(const bMatch of bracketMatches){
        const expected = `${bMatch[1]}${bMatch[0]}${bMatch[1]}`
        if(lineNoBrackets.includes(expected)){
            return true;
        }
    }

    return false
}

const part2 = input.split('\n').filter(line =>  isSSLSupported(line))

console.log(part2)
const ansPart2 = part2.length

console.log(`Part 2: ${ansPart2}`)
