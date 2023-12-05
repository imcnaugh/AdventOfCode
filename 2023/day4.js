import fs from 'fs'

const isTest = false
const fileName = `day4${isTest ? '.test' : ''}.txt`
const input = fs.readFileSync(`resources/${fileName}`, 'utf-8')

function getNumbersFromString(s, delim = ' ') {
  return s.split(delim).filter(s => s).map(a => parseInt(a))
}

let copy = []
const lines = input.split('\n').length
for(let g = 0; g < lines; g++){
  copy.push(1)
}

const output = input.split('\n').map(line => {
  const copies = copy[0]
  copy = copy.splice(1, copy.length)

  const sMap = {}
  let w = 0

  const selectedAndWinners = line.split(':')[1].split('|')
  const gameNumber = line.split(':')[0]
  const selected = getNumbersFromString(selectedAndWinners[0])
  const winners = getNumbersFromString(selectedAndWinners[1])

  selected.forEach(s => sMap[s] = true)
  winners.forEach(a => sMap[a] === true ? w++ : null)

  for(let i = 0; i < w; i++){
    copy[i] = copy[i] + copies
  }

  return copies
}).reduce((p, c) => c + p, 0)

console.log(output)
