import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

const indexedMaps = new Array(8).fill(null)
for(let i = 0; i < 8; i++){
    indexedMaps[i] = new Map()
}

input.split('\n').forEach(line => {
    for(let i = 0; i < 8; i++){
        const map = indexedMaps[i]
        const char = line[i]

        if(!map.has(char)){
            map.set(char, 0)
        }
        map.set(char, map.get(char) + 1)
    }
})

let part1 = indexedMaps.map(m => {
    const sorted =  Array.from(m).sort((a, b) => a[1] - b[1])
    return sorted[0][0]
}).join('')

console.log(`Part 1: ${part1}`)