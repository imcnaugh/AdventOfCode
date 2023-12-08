import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`./resources/day8${isTest ? '.test' : ''}.txt`, 'utf8')

const lines = input.split('\n')

const instructions = lines[0].split('')
const m = lines.splice(2)

const map = {}
m.forEach(line => {
  const parts = line.split('=').map(a => a.trim())
  const options = parts[1].slice(1, -1).split(', ').map(a => a.trim())
  map[parts[0]] = [options[0], options[1]]
})

let currentNode = 'AAA'
let stepCount = 0

while(currentNode !== 'ZZZ'){
  const dir = instructions[stepCount % instructions.length]
  currentNode = map[currentNode][dir === 'L' ? 0 : 1]
  stepCount++
}

console.log(stepCount)