import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day6${isTest ? '.test' : ''}.txt`, 'utf-8').replace(/ +/g,  ' ')

const lines = input.split('\n').map(line => line.split(' ').splice(1).map(Number))

const races = Array.from({length: lines[0].length}, () => [])
lines.forEach(l => l.forEach((n, i) => races[i].push(n)))

const output = races.map(r => {
  let wins = 0
  for(let h = 1; h < r[0]; h++){
    const timeRemaining = (r[0]-h)
    if((h * timeRemaining) > r[1]) wins++
  }
  return wins
}).reduce((c, p) => c * p)

console.log(output)