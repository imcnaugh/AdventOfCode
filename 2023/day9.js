import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day9${isTest ? '.test' : ''}.txt`, 'utf-8')

const isSequenceAllZeros = (seq) =>  seq.reduce((p, c) => p && c === 0, true)

const generateNextSequence = (numArr) => {
  let output = []
  for(let i = 1; i < numArr.length; i++){
    output.push(numArr[i] - numArr[i-1])
  }
  return output
}

const goDeeper = (seq) => {
  if(isSequenceAllZeros(seq)) return 0
  const nextSeq = generateNextSequence(seq)
  const carry = goDeeper(nextSeq)
  return seq[0] - carry
}

const output = input.split('\n').map(line => {
  const initialNumbers = line.split(' ').map(Number)
  return goDeeper(initialNumbers)
}).reduce((p,c) => p+c)

console.log(output)