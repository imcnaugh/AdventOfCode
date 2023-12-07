import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day7${isTest?'.test':''}.txt`, 'utf-8')

const getCardNum = (c) => {
  switch(c){
    case 'A': return 14
    case 'T': return 10
    case 'J': return -1
    case 'Q': return 12
    case 'K': return 13
    default: return parseInt(c)
  }
}

const getType = (cards) => {
  cards = cards.filter(c => c !== -1)
  const groupings = cards.reduce((pre, current) => {
    if(!pre[current]) pre[current] = 0
    pre[current] = pre[current] + 1
    return pre
  }, {})
  const jokerCount = 5 - cards.length
  let maxCount = 0
  let maxNum
  for(const c in groupings){
    if(groupings[c] > maxCount){
      maxCount = groupings[c]
      maxNum = c
    }
  }

  groupings[maxNum] = groupings[maxNum] + jokerCount

  let has3ofAKind = false
  let has2ofAKind = false
  let countOfParis = 0
  for(const c in groupings){
    if(groupings[c] === 5){
      return 10
    }
    if(groupings[c] === 4){
      return 9
    }
    if(groupings[c] === 3){
      has3ofAKind = true
    }
    if(groupings[c] === 2){
      countOfParis++
      has2ofAKind = true
    }
  }

  if(has3ofAKind && has2ofAKind) return 8
  if(has3ofAKind) return 7
  if(countOfParis === 2) return 6
  if(has2ofAKind) return 5
  return 4
}


const sortHand = (aHaB, bHaB) => {
  const a = aHaB.split(' ')[0].split('').map(getCardNum)
  const b = bHaB.split(' ')[0].split('').map(getCardNum)

  const aRank = getType(a)
  const bRank = getType(b)

  if(aRank !== bRank) return aRank - bRank

  for(let i = 0; i < 5; i++){
    if(a[i] !== b[i]) return a[i] - b[i]
  }
  return 0
}

const output = input.split('\n')
  .sort(sortHand)
  .map(hab => parseInt(hab.split(' ')[1]))
  .reduce((pre, cur, index) => pre + (cur * (index+1)), 0)

console.log(output)