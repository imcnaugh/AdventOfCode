import fs from 'fs'


const test = 0
const fileName = `day3${test? '.test' : ''}.txt`
const input = fs.readFileSync(`resources/${fileName}`, 'utf-8')

const lines = input.split('\n')
let digit = ''
let isDigit = false
let isValid = false

let sum = 0

function isCharASymbol(l, c){
  let char = lines[l].charAt(c);
  return !(char === '.' || char.match(/\d/))
}

function checkAboveAndBelow(l, c){
  let top = isCharASymbol(Math.max(0, l-1), c)
  let bottom = isCharASymbol(Math.min(l+1, lines.length-1), c)
  return top || bottom
}

function addDigitToSum(){
  sum += parseInt(digit)
}

for(let lineNum = 0; lineNum < lines.length; lineNum++){
  const line = lines[lineNum]
  for(let charNum = 0; charNum < line.length; charNum++){
    if(line.charAt(charNum).match(/\d/)){
      if(!isDigit && charNum !== 0) {
        isValid = checkAboveAndBelow(lineNum, charNum - 1)
        isValid = isValid || isCharASymbol(lineNum, charNum - 1)
      }
      digit += line.charAt(charNum)
      isDigit = true
      isValid = isValid || checkAboveAndBelow(lineNum, charNum)
    }
    else {
      if(isDigit){
        if(charNum < line.length){
          isValid = isValid || checkAboveAndBelow(lineNum, charNum)
          isValid = isValid || isCharASymbol(lineNum, charNum)
        }

        if(isValid) addDigitToSum()
        isValid = false
        isDigit = false
        digit = ''
      }
    }
  }
  if(isValid) addDigitToSum()
  isValid = false
  isDigit = false
  digit = ''
}

console.log(sum)