import fs from 'fs'

const isTest = true
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

let memo = {}

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
  if(memo[prefix]) return memo[prefix]
  if(sig.length === 0){
    for(let i = 0; i < record.length; i++){
      const recordChar = record[i]
      const prefixChar = prefix[i]
      if(recordChar === '?'){

      } else if (recordChar !== prefixChar){
        return 0
      }
    }
    return 1
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

    const good = generateAll(record, nexSig, plusPrefix)
    memo[plusPrefix] = good
    totalGood = totalGood + good
  }

  return totalGood
}

const getUnfoldedRecords = (r) => {
  let big = r
  for(let i = 0; i < 4; i++){
    big += '?'
    big += r
  }
  return big
}

const getUnfoldedSig = (s) => {
  let big = s
  for(let i = 0; i < 4; i++) {
    big += ','
    big += s
  }
  return big
}


const getNumberOfVariations = (line) => {
  const parts = line.split(' ')

  const unfoldedRecord = getUnfoldedRecords(parts[0])
  const unfoldedSig = getUnfoldedSig(parts[1])

  const conditionRecord = unfoldedRecord.replace(/^\.+|\.+$/g, '').split('')
  const signature = unfoldedSig.split(',').map(Number)
  const{currentRecord: trimmedRecord, currentSig: trimmedSig} = reduceStartAndEnd(conditionRecord, signature)
  const varients =  generateAll(trimmedRecord, trimmedSig, '')
  console.log(line, varients)
  return varients
}

const output = input.split('\n').map(getNumberOfVariations).reduce((p,c) => p+c)

console.log(`output: ${output}`)