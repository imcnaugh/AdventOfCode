import fs from 'fs'

const input = fs.readFileSync('resources/day2.txt', 'utf-8')

const expected = {
  red: 12,
  green: 13,
  blue: 14
}

function parseGame(line){
  let sep = line.indexOf(':');
  const gameNumber = line.substring(5, sep)
  const pulls = line.substring(sep + 1, line.length).split(';').map(p => p.trim())
  const allValidPulls = pulls.map(pull =>{
    return pull.split(',').map(colCount => colCount.trim()).map(cc => {
      const idk = cc.split(' ')
      return parseInt(expected[idk[1]]) >= parseInt(idk[0])
    }).reduce((previous, current) => (previous && current), true)
  }).reduce((previous, current) => (previous && current), true)
  return allValidPulls ? gameNumber : 0
}

const output = input.split('\n').map(game =>{
  return parseInt(parseGame(game))
}).reduce((p, c) => p + c, 0)

console.log(output)