import fs from 'fs'

const isTest = true
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

let memo = {};

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

function generateAll(record, sig) {
  let possibilities = 0
  // Strip prefixing '.'
  while (record.length > 0 && record[0] === '.') {
    record = record.slice(1);
  }

  // if record is empty but sig still has elements, we have an invalid possibility
  if (record.length === 0 && sig.length !== 0){
    return 0
  }

  if (sig.length === 0){
    // if sig is empty and there are no more known broken springs, return possibilities
    if (record.includes('#')) {
      return 0
    } else {
      return 1
    }
  }

  let length_of_next_possible;
  for (length_of_next_possible = 0; length_of_next_possible < record.length; length_of_next_possible++){
    if (record[length_of_next_possible] === '.') {
      break;
    }
  }

  if (length_of_next_possible < sig[0]) {
    let sub_record = record.slice(length_of_next_possible)
    possibilities += generateAll(sub_record, sig)
  } else {
    let new_sig = sig.slice(1);

    for (let i = 0; i + sig[0] <= length_of_next_possible; i++){
      let new_record = record.slice(sig[0] + i + 1);
      possibilities += generateAll(new_record, new_sig)
    }
  }
  return possibilities;
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

const map_to_possible_broken_map = (s) => {
  s = s.replaceAll(/\.+/g, '.');


}


const getNumberOfVariations = (line) => {
  const parts = line.split(' ')

  // const unfoldedRecord = getUnfoldedRecords(parts[0])
  // const unfoldedSig = getUnfoldedSig(parts[1])

  const unfoldedRecord = parts[0]
  const unfoldedSig = parts[1]

  const conditionRecord = unfoldedRecord.replace(/^\.+|\.+$/g, '').split('')
  const signature = unfoldedSig.split(',').map(Number)
  const{currentRecord: trimmedRecord, currentSig: trimmedSig} = reduceStartAndEnd(conditionRecord, signature)
  const varients =  generateAll(trimmedRecord, trimmedSig, 0)
  console.log(line, varients)
  return varients
}

const output = input.split('\n').map(getNumberOfVariations).reduce((p,c) => p+c)

console.log(`output: ${output}`)