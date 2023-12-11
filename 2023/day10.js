import fs from 'fs'

const isTest = false
let input = fs.readFileSync(`resources/day10${isTest ? '.test' : ''}.txt`, 'utf-8')

class Pipe {
  constructor(char, x, y){
    this.id = `${x}:${y}`
    this.char = char
    this.x = x
    this.y = y
  }
}


let charMap = {}

input.split('\n').map((line, y) => line.split('').map((char, x) => {
  const newSegment = new Pipe(char, x , y)
  charMap[newSegment.id] = newSegment
}))

let start

const setConnectors = (segment) => {
  const x = segment.x
  const y = segment.y

  const setC = (a, b) => {
    const connections = [a, b]
    segment.connections = connections.filter(a => a)
  }

  const north = charMap[`${x}:${y-1}`]
  const east = charMap[`${x+1}:${y}`]
  const south = charMap[`${x}:${y+1}`]
  const west = charMap[`${x-1}:${y}`]

  switch(segment.char){
    case '|': {
      setC(north, south)
      break
    }
    case '-': {
      setC(east, west)
      break
    }
    case 'L': {
      setC(north, east)
      break
    }
    case 'J': {
      setC(north, west)
      break
    }
    case '7': {
      setC(west, south)
      break
    }
    case 'F': {
      setC(south, east)
      break
    }
    case 'S': {
      segment.connections = []
      const northChar = north.char
      const eastChar = east.char
      const southChar = south.char
      const westChar = south.char
      if(northChar === '|' || northChar === '7' || northChar === 'F') segment.connections.push(north)
      if(eastChar === '-' || eastChar === 'J' || eastChar === '7') segment.connections.push(east)
      if(southChar === '|' || southChar === 'L' || southChar === 'J') segment.connections.push(south)
      if(westChar === '-' || westChar === '7' || westChar === 'J') segment.connections.push(west)
      start = segment
    }
  }
}

for(const segment in charMap){
  setConnectors(charMap[segment])
}

let previous = start
let current = start.connections[0]
let mainLoop = {}
mainLoop[start.id] = isTest ? 'F' : 'L'

while(current.id !== start.id){
  mainLoop[current.id] = current.char
  const next = current.connections.find(c => c.id !== previous.id)
  previous = current
  current = next
}


const width = input.split('\n')[0].split('').length
const height = input.split('\n').length

let inMain = false
let count = 0
let inChar = ''

for(let y = 0; y < height; y++){
  for(let x = 0; x < width; x++){
    const ml = mainLoop[`${x}:${y}`]
    if(inMain && ml === undefined) {
      count++
    }
    if(!inMain){
      if(ml === '|' || ml === 'F' || ml === 'L'){
        inChar = ml
        inMain = true
      }
    } else {
      if(inChar === '|'){
        if(ml === '|'){
          inMain = false
          inChar = undefined
        } else if(ml === 'F') {
          inChar = 'L'
        } else if(ml === 'L') {
          inChar = 'F'
        }
      } else if(inChar === 'F' && (ml === '|' || ml === '7')){
        inMain = false
        inChar = undefined
      } else if(inChar === 'L' && (ml === '|' || ml === 'J')){
        inMain = false
        inChar = undefined
      }
    }
  }
  inMain = false
}

console.log(count)
