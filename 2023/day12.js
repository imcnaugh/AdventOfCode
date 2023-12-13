import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

const reduceStartAndEnd = (conditionRecord, sig) => {
  let currentRecord = [conditionRecord[0]]
  let currentSig = sig

  let previousChar = conditionRecord[0]
  for(let i = 1; i < conditionRecord.length; i++){
    if(previousChar === '.' && conditionRecord[i] === '.'){

    } else {
      previousChar = conditionRecord[i]
      currentRecord.push(conditionRecord[i])
    }
  }

  return {currentRecord, currentSig}
}

function generateAll(record, sig, prefix) {
  if(sig.length === 0){
    console.log(prefix)
    for(let i = 0; i < record.length; i++){
      const recordChar = record[i]
      const prefixChar = prefix[i]
      if(recordChar === '?'){

      } else if (recordChar !== prefixChar){
        return 0
      }
    }
    return 1
    // base case, check if prefix is a valid record, return 1 if yes, 0 if no
  }

  let maxRemaining = sig[0]
  for(let i = 1; i < sig.length; i++){
    maxRemaining += sig[i] + 1
  }

  let totalGood = 0
  let currentCount = sig[0]
  let nexSig = []
  for(let i = 1; i < sig.length; i++){
    nexSig.push(sig[i])
  }

  for(let i = 0; (i + maxRemaining + prefix.length) <= record.length; i++){
    let str = '.'.repeat(i) + '#'.repeat(currentCount)
    if(nexSig.length > 0) str = str + '.'
    let plusPrefix = prefix + str

    if(nexSig.length === 0) plusPrefix = plusPrefix.padEnd(record.length, '.')

    totalGood = totalGood + generateAll(record, nexSig, plusPrefix)
  }

  return totalGood
}


const getNumberOfVariations = (line) => {
  const parts = line.split(' ')
  const conditionRecord = parts[0].replace(/^\.+|\.+$/g, '').split('')
  const signature = parts[1].split(',').map(Number)
  const{currentRecord: trimmedRecord, currentSig: trimmedSig} = reduceStartAndEnd(conditionRecord, signature)
  const varients =  generateAll(trimmedRecord, trimmedSig, '')
  console.log(line, varients)
  return varients
}

const output = input.split('\n').map(getNumberOfVariations).reduce((p,c) => p+c)

console.log(`output: ${output}`)