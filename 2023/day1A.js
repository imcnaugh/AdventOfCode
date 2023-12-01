import fs from 'fs'

const numbers = [
  {str:'0', value:0},
  {str:'1', value:1},
  {str:'2', value:2},
  {str:'3', value:3},
  {str:'4', value:4},
  {str:'5', value:5},
  {str:'6', value:6},
  {str:'7', value:7},
  {str:'8', value:8},
  {str:'9', value:9},
  {str:'one', value:1},
  {str:'two', value:2},
  {str:'three', value:3},
  {str:'four', value:4},
  {str:'five', value:5},
  {str:'six', value:6},
  {str:'seven', value:7},
  {str:'eight', value:8},
  {str:'nine', value:9},
]

function startsWithNumber(line) {

  for(const possibility of numbers) {
    if(line.startsWith(possibility.str)){
      return possibility.value
    }
  }
}

const getCodeFromLine = (line) => {
  let first
  let last
  
  for(let i = 0; i < line.length; i++){
    const subLine = line.substring(i)
    const maybe = startsWithNumber(subLine)
    if(maybe){
      first = first || maybe
      last = maybe
    }
  }

  return parseInt(`${first}${last}`)
}

const input = fs.readFileSync('resources/20231201A.txt', 'utf-8')

const output = input.split('\n')
  .map(getCodeFromLine)
  .reduce((previous, current) => previous + current, 0)
console.log(output)