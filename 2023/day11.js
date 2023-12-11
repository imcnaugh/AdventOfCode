import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day11${isTest ? '.test' : ''}.txt`, 'utf-8')

const map = {}

input.split('\n').map((line, y) => line.split('').map((char, x) => {
  const id = `${x}:${y}`
  map[id] = char
}))

const doubleRows = []
const doubleCols = []

const ix = input.split('\n')[0].length
const iy = input.split('\n').length

for(let x = 0; x < ix; x++){
  let allPeriods = true
  for(let y = 0; y < iy; y++){
    const id = `${x}:${y}`
    if(map[id] !== '.') allPeriods = false
  }
  if(allPeriods) doubleCols.push(x)
}

for(let y = 0; y < iy; y++){
  let allPeriods = true
  for(let x = 0; x < ix; x++){
    const id = `${x}:${y}`
    if(map[id] !== '.') allPeriods = false
  }
  if(allPeriods) doubleRows.push(y)
}

const gals = Object.keys(map).filter(id => map[id] === '#')

let totalDistance = 0

for(let ia = 0; ia < gals.length; ia++){
  for(let ib = ia + 1; ib < gals.length; ib++){
    const gal1 = gals[ia]
    const gal2 = gals[ib]

    const gal1X = gal1.split(':')[0]
    const gal1Y = gal1.split(':')[1]
    const gal2X = gal2.split(':')[0]
    const gal2Y = gal2.split(':')[1]

    const minX = Math.min(gal1X, gal2X)
    const maxX = Math.max(gal1X, gal2X)

    const minY = Math.min(gal1Y, gal2Y)
    const maxY = Math.max(gal1Y, gal2Y)

    for(let x = minX; x < maxX; x++){
      if(doubleCols.find(a => a === x)) totalDistance = totalDistance + 1000000
      else totalDistance++
    }

    for(let y = minY; y < maxY; y++){
      if(doubleRows.find(a => a === y)) totalDistance = totalDistance + 1000000
      else totalDistance++
    }
  }
}

console.log(totalDistance)