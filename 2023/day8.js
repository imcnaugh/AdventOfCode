import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`./resources/day8${isTest ? '.test' : ''}.txt`, 'utf8')

const lines = input.split('\n')

const instructions = lines[0].split('')
const mapLines = lines.splice(2)
const getDirection = (i) => i === 'L' ? 0 : 1

const map = {}
const startingNodes = []

mapLines.forEach(line => {
  const parts = line.split('=').map(a => a.trim())
  const options = parts[1].slice(1, -1).split(', ').map(a => a.trim())
  const nodeName = parts[0]
  for(let i = 0 ; i < instructions.length; i++) {
    const instruction = instructions[i]
    const nodeNameAndInstruction = `${i}:${nodeName}`
    const next = options[getDirection(instruction)]
    const nextInstructionNumber = (i + 1) % instructions.length
    const nextNodeName = `${nextInstructionNumber}:${next}`
    map[nodeNameAndInstruction] = nextNodeName
  }
  if(nodeName[2] === 'A') startingNodes.push(`0:${nodeName}`)
})

const getCountToNextOut = (start) => {
  let count = 0
  do{
    count++
    start = map[start]
  }while(!start.endsWith('Z'))
  return {next: start, count: count}
}

const startAndNext = startingNodes.map(node => {
  const {next, count} = getCountToNextOut(node)
  return {start: node, next: next, count: count}
})

const outAndNext = {}
Object.keys(map).filter(k => k.endsWith('Z')).map(node => {
  const {next, count} = getCountToNextOut(node)
  outAndNext[node] = {next: next, count: count}
})

const nodes = startAndNext.map(san => {
  return {next: san.next, count: san.count }
})

const allNodesHaveSameCount = (nodes) => {
  const n0Count = nodes[0].count
  return nodes.reduce((p,c) => p && (c.count === n0Count), true)
}

const getMinNodeIndex = (nodes) => {
  let low = Number.MAX_SAFE_INTEGER
  let ind = NaN
  for(let i = 0 ; i < nodes.length; i++){
    if(nodes[i].count < low){
      low = nodes[i].count
      ind = i
    }
  }
  return ind
}

while(!allNodesHaveSameCount(nodes)){
  const lowNodeIndex = getMinNodeIndex(nodes)
  const lowNodeNextNode = nodes[lowNodeIndex].next
  const lowNodeCount = nodes[lowNodeIndex].count
  const next = outAndNext[lowNodeNextNode]
  const totalCount = lowNodeCount + next.count
  nodes[lowNodeIndex] = {next: next.next, count: totalCount}
}

console.log(nodes[0].count)