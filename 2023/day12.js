import fs from 'fs'

const isTest = true
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

let memo = {};

function generateAll(record, sig, record_index, sig_index, curr_dam_length, possible) {
  if (record.length === record_index && sig.length === sig_index) {
    return possible;
  }
  if (record_index >= record.length || sig_index >= sig.length) {
    return 0;
  }
  
  if (record[record_index] === ".") {
    let possible_positions = curr_dam_length - sig[sig_index] + 1;
    if (possible_positions < 0) return 0;
    return generateAll(record, sig, record_index + 1, sig_index + 1, 0, possible + possible_positions)
  } else if (record[record_index] === "#") {
    let next_not = record_index;
    while (next_not < record.length && record[next_not] === "#") {
      next_not++;
    }
    return generateAll(record, sig, record_index + next_not, sig_index, curr_dam_length + next_not, possible)
  } else if (record[record_index] === "?") {
    let total = 0;
    let dot_record = record.slice()
    dot_record[record_index] = "."
    let hash_record = record.slice()
    hash_record[record_index] = "#"

    const prevChar = record_index > 0 ? record[record_index - 1] : null;
    total += generateAll(dot_record, sig, record_index + 1, (prevChar === '#'? sig_index + 1: sig_index), 0, possible)
    total += generateAll(hash_record, sig, record_index + 1, sig_index, curr_dam_length + 1, possible)
    return total;
  }
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
  memo = {}
  const parts = line.split(' ')

  // const unfoldedRecord = getUnfoldedRecords(parts[0])
  // const unfoldedSig = getUnfoldedSig(parts[1])

  const unfoldedRecord = parts[0]
  const unfoldedSig = parts[1]

  const conditionRecord = unfoldedRecord.split('')

  conditionRecord.push('.');


// Remove all '.' from the start of conditionRecord if they exist
  while (conditionRecord[0] === '.') {
    conditionRecord.shift();
  }
  
  const signature = unfoldedSig.split(',').map(Number)
  const varients =  generateAll(conditionRecord, signature, 0, 0 , 0, 0)
  console.log(line, varients)
  return varients
}

const output = input.split('\n').map(getNumberOfVariations).reduce((p,c) => p+c)

console.log(`output: ${output}`)