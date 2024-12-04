import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

let memo = {};

function generateAll(record, sig, index) {
  const key = record.join("") + index;
  if (memo.hasOwnProperty(key)) return memo[key];

  if (index >= record.length) {
    const valid = check(record.join(""), sig) ? 1 : 0;
    memo[key] = valid;
    return valid;
  }

  let possible = 0;

  if (record[index] === "?") {
    let dot_record = record.slice();
    dot_record[index] = "."
    let hash_record = record.slice();
    hash_record[index] = "#";
    possible += generateAll(dot_record, sig, index + 1)
    possible += generateAll(hash_record, sig, index + 1)
  } else {
    possible += generateAll(record, sig, index + 1);
  }
  memo[key] = possible; // Cache the result before returning
  return possible;
}

const check = (record, sig) => {
  record = record + ".";
  let sig_index = 0;

  let in_broken;
  let broken_count = 0;

  for (let i = 0; i < record.length; i++) {
    if (record[i] === "#") {
      in_broken = true;
      broken_count++;
    } else {
      if (in_broken) {
        if (sig_index >= sig.length || sig[sig_index] !== broken_count) {
          return false;
        }
        sig_index++;
      }
      broken_count = 0;
      in_broken = false;
    }
  }

  return sig_index === sig.length;
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

  const unfoldedRecord = getUnfoldedRecords(parts[0])
  const unfoldedSig = getUnfoldedSig(parts[1])

  // const unfoldedRecord = parts[0]
  // const unfoldedSig = parts[1]

  const conditionRecord = unfoldedRecord.split('')
  const signature = unfoldedSig.split(',').map(Number)
  const varients =  generateAll(conditionRecord, signature, 0)
  console.log(line, varients)
  return varients
}

const output = input.split('\n').map(getNumberOfVariations).reduce((p,c) => p+c)

console.log(`output: ${output}`)