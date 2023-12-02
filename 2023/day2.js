import fs from 'fs'

const input = fs.readFileSync('resources/day2.txt', 'utf-8')


function parseGame(line){
  const colorMax = {}
  let sep = line.indexOf(':');
  const pulls = line.substring(sep + 1, line.length).split(';').map(p => p.trim())
  pulls.map(pull =>{
    return pull.split(',').map(colCount => colCount.trim()).map(cc => {
      const idk = cc.split(' ')
      if(!colorMax[idk[1]] || colorMax[idk[1]] < parseInt(idk[0])){
        colorMax[idk[1]] = parseInt(idk[0])
      }
    })
  })
  let prod = 1
  for(const c in colorMax){
    prod = prod * parseInt(colorMax[c])
  }
  return prod
}

const output = input.split('\n').map(game =>{
  return parseInt(parseGame(game))
}).reduce((p, c) => p + c, 0)

console.log(output)