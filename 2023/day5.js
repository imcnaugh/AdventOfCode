import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day5${isTest? '.test' : ''}.txt`, 'utf-8')

const sections = input.split('\n\n')

const seeds = sections[0].split(':')[1].split(' ').filter(a => a).map(a => parseInt(a))
let seedRanges = []
for(let i = 0; i < seeds.length; i = i + 2){
  seedRanges.push([seeds[i], seeds[i+1]])
}

const getMappings = (line) => line.split(':')[1]
  .split('\n')
  .filter(a => a)
  .map(m => {
    const l = m.split(' ').filter(a => a).map(a => parseInt(a))
    return {
      range: [l[1], l[1] + l[2] - 1],
      op: l[0] - l[1]
    }
  }).sort((a,b) => a.range[0] - b.range[0])

const mappings = sections.slice(1).map(getMappings)

let i = []
mappings.reverse().map(m => {
  let newI = []
  i.forEach(oldI => {
    const o = m.find(r => (r.range[0] + r.op) <= oldI && (r.range[1] + r.op) >= oldI)
    newI.push( o ? oldI - o.op : oldI)
  })
  m.forEach(r => newI.push(r.range[0]))
  i = newI
})

mappings.reverse()

const indexAndOutputs = i.map(i => {
  let carry = i
  mappings.forEach(m =>{
    const t = m.find(x => (x.range[0] <= carry) && (x.range[1] >= carry))
    if(t) carry = carry + t.op
  })
  return {
    index: i,
    output: carry
  }
})

indexAndOutputs.sort((a,b) => a.index - b.index)
seedRanges.sort((a, b) => a[0] - b[0])

let lowest = Number.MAX_SAFE_INTEGER
seedRanges.map(r => {
  const low = r[0]
  const high = r[1]
  for(let i = low; i < high; i++){
    const closestIndex = indexAndOutputs.find(a => a.index < r)
  }
})

console.log(seedRanges)
console.log(indexAndOutputs)