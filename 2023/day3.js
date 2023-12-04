import fs from 'fs'


const test = 0
const fileName = `day3${test? '.test' : ''}.txt`
const input = fs.readFileSync(`resources/${fileName}`, 'utf-8')

const lines = input.split('\n')
let digit = ''
let isDigit = false
let isValid = false

let gearLine = null
let gearChar = null

let gears = {}

function isCharASymbol(l, c){
  if(lines[l].charAt(c).match(/\*/)){
    gearLine = l
    gearChar = c
  }
}

function checkAboveAndBelow(l, c){
  isCharASymbol(Math.max(0, l-1), c)
  isCharASymbol(Math.min(l+1, lines.length-1), c)
}

function addNumberToGear(){
  if(gearLine === null ) return
  const index = `${gearLine}:${gearChar}`
  if(!gears[index]){
    gears[index] = []
  }
  gears[index].push(parseInt(digit))
}


for(let lineNum = 0; lineNum < lines.length; lineNum++){
  const line = lines[lineNum]
  for(let charNum = 0; charNum < line.length; charNum++){
    if(line.charAt(charNum).match(/\d/)){
      if(!isDigit && charNum !== 0) {
        checkAboveAndBelow(lineNum, charNum - 1)
        isCharASymbol(lineNum, charNum - 1)
      }
      digit += line.charAt(charNum)
      isDigit = true
      checkAboveAndBelow(lineNum, charNum)
    }
    else {
      if(isDigit){
        if(charNum < line.length){
          checkAboveAndBelow(lineNum, charNum)
          isCharASymbol(lineNum, charNum)
        }
        addNumberToGear()
        isDigit = false
        gearLine = null
        gearChar = null
        digit = ''
      }
    }
  }
  addNumberToGear()
  isDigit = false
  gearLine = null
  gearChar = null
  digit = ''
}

console.log(gears)

let sum = 0
for(const g in gears){
  if(gears[g].length === 2){
    const p = gears[g][0] * gears[g][1]
    sum += p
  }
}

console.log(sum)