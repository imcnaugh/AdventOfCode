import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')
// const input = "(2x2)bcd"

let runningLen = 0

for(let i = 0; i < input.length; i++){
    if(input[i] === '('){
        i++
        let distStr = []
        let multStr = []
        let popDist = true
        while(input[i] !== ')'){
            if(input[i] === 'x'){
                popDist = false
            } else if(popDist) {
                distStr.push(input[i])
            } else {
                multStr.push(input[i])
            }
            i++
        }
        const dist = parseInt(distStr.join(''))
        const mult = parseInt(multStr.join(''))
        const remainder = input.length - i;
        i += dist
        const distIdk = Math.min(dist, remainder)
        runningLen += (distIdk * mult)
    } else {
        runningLen++
    }
}

console.log(`Part 1: ${runningLen}`)