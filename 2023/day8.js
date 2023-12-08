import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`./resources/day8${isTest ? '.test' : ''}.txt`, 'utf8')

const lines = input.split('\n')

const instructions = lines[0].split('')
const m = lines.splice(2)

const startingNodes = []

const map = {}
m.forEach(line => {
  const parts = line.split('=').map(a => a.trim())
  const options = parts[1].slice(1, -1).split(', ').map(a => a.trim())
  map[parts[0]] = [options[0], options[1]]
  if(parts[0][parts[0].length - 1] === 'A') startingNodes.push([parts[0]])
})

const strEndsInZ = (s) => s[2] === 'Z'
const allNodesEndInZ = (n) => n.reduce((p, c) => p && strEndsInZ(c), true)

let nodes = startingNodes

let stepCount = 0
while(!allNodesEndInZ(nodes)){
  const dir = instructions[stepCount % instructions.length]
  nodes = nodes.map(node => {
    return map[node][dir === 'L' ? 0 : 1]
  })
  stepCount++
  if(stepCount % 10000000 === 0) console.log(stepCount)
}

console.log('finished: ', stepCount)