import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

function generateChecksum(name){
    const map = new Map()
    name.split('').forEach(c => {
        if(c !== '-'){
            if (!map.has(c)) {
                map.set(c, 0)
            }
            map.set(c, map.get(c) + 1)
        }
    })

    const idk = Array.from(map).map(e => {
        return {char: e[0], count: e[1]}
    }).sort((a, b) => {
        if(a.count === b.count){
            return a.char.localeCompare(b.char)
        }
        return b.count -g a.count
    })
    return idk.slice(0, 5).map(a => a.char).join('')
}

const parsingRegex = /^([a-z-]*)(\d*)\[([a-z]*)]/
const sectorIdSum = input.split('\n').map(line => {
    const matches = line.match(parsingRegex)
    return {
        name: matches[1],
        sectorId: parseInt(matches[2]),
        checkSum: matches[3]
    }
}).filter(row => {
    return generateChecksum(row.name) === row.checkSum
}).map(row => row.sectorId).reduce((acc, c) => acc + c, 0)

console.log(`Part 1: ${sectorIdSum}`)