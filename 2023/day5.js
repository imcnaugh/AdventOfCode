import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day5${isTest? '.test' : ''}.txt`, 'utf-8')

const sections = input.split('\n\n')

const seeds = sections[0].split(':')[1].split(' ').filter(a => a).map(a => parseInt(a))
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

let seedRanges = []
for(let i = 0; i < seeds.length; i = i + 2){
  seedRanges.push([seeds[i], seeds[i+1]])
}

let lowest = Number.MAX_SAFE_INTEGER

seedRanges.map(seedRange =>{
  console.log('current seed range ', seedRange)
  const high = seedRange[0] + seedRange[1]
  for(let sr = seedRange[0]; sr < high; sr++){
    let carry = sr
    mappings.forEach(level => {
      const map = level.find(l => (l.range[0] <= carry) && (l.range[1] >= carry))
      if(map){
        carry = carry + map.op
      }
    })
    lowest = Math.min(lowest, carry)
  }
})

console.log(lowest)


