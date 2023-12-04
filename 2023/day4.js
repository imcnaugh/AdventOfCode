import fs from 'fs'

const isTest = false
const fileName = `day4${isTest ? '.test' : ''}.txt`
const input = fs.readFileSync(`resources/${fileName}`, 'utf-8')

function getNumbersFromString(s, delim = ' ') {
  return s.split(delim).filter(s => s).map(a => parseInt(a))
}

const output = input.split('\n').map(line => {
  const sMap = {}
  let w = 0

  const selectedAndWinners = line.split(':')[1].split('|')
  const selected = getNumbersFromString(selectedAndWinners[0])
  const winners = getNumbersFromString(selectedAndWinners[1])

  selected.forEach(s => sMap[s] = true)
  winners.forEach(a => sMap[a] === true ? w++ : null)
  return w ? Math.pow(2, w - 1) : 0
}).reduce((p, c) => c + p, 0)

console.log(output)