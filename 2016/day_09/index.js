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
        i += dist
        runningLen += (dist * mult)
    } else {
        runningLen++
    }
}

console.log(`Part 1: ${runningLen}`)

function unfoldAndGetLength(input, start, end) {
    let len = 0
    let i = start
    while(i < end){
        if(input[i] === '('){
            let dist = []
            let mult = []

            let curPos = i + 1 // account for first '('
            let popDist = true
            while(input[curPos] !== ')'){
                if(input[curPos] === 'x') popDist = false
                else popDist ? dist.push(input[curPos]) : mult.push(input[curPos])
                curPos++
            }
            curPos++ // account for end ')'

            dist = parseInt(dist.join(''))
            mult = parseInt(mult.join(''))

            len += unfoldAndGetLength(input, curPos, curPos + dist) * mult
            i = curPos + dist
        } else {
            len++
            i++
        }
    }

    return len
}

console.log(`Part 2: ${unfoldAndGetLength(input, 0, input.length)}`)