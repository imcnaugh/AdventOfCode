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

    const sortedChars = Array.from(map).map(e => {
        return {char: e[0], count: e[1]}
    }).sort((a, b) => {
        if(a.count === b.count){
            return a.char.localeCompare(b.char)
        }
        return b.count - a.count
    })
    return sortedChars.slice(0, 5).map(a => a.char).join('')
}

function decryptName(name, sectorId) {
    let rot = sectorId % 26;
    return name.split('').map(c => {
        if(c === '-')
            return '-'
        let a = c.charCodeAt(0) + rot
        if(a > 'z'.charCodeAt(0)){
            a -= 26
        }
        return String.fromCharCode(a)
    }).join('')
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

const expected = /northpole-object/i
// const expected = /north/i

input.split('\n').map(line => {
    const matches = line.match(parsingRegex)
    return {
        name: matches[1],
        sectorId: parseInt(matches[2]),
        checkSum: matches[3]
    }
}).forEach(line => {
    let decrypted = decryptName(line.name, line.sectorId)
    if(expected.test(decrypted)){
        console.log(`Part 2: ${decrypted} ${line.sectorId}`)
    }
})