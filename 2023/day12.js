import fs from 'fs'

const isTest = true
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

const reduceConditionRecord = (conditionRecord) => {
  let char = conditionRecord[0]
  let count = 1
  let output = []
  for(let i = 1; i < conditionRecord.length; i++){
    if(char === conditionRecord[i]) {
      count++
    } else {
      output.push({char: char, count: count})
      char = conditionRecord[i]
      count = 1
    }
  }
  output.push({char: char, count: count})
  return output
}

const getNumberOfVariations = (line) => {
  const parts = line.split(' ')
  const conditionRecord = parts[0].replace(/^\.+|\.+$/g, '').split('')
  const reducedConditionRecord = reduceConditionRecord(conditionRecord)
  const signature = parts[1].split(',').map(Number)



  console.log(reducedConditionRecord, signature)
}

input.split('\n').map(getNumberOfVariations)
