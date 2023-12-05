import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day5${isTest? '.test' : ''}.txt`, 'utf-8')

const sections = input.split('\n\n')

const seeds = sections[0].split(':')[1].split(' ').filter(a => a).map(a => parseInt(a))
const getMappings = (line) => line.split(':')[1].split('\n').filter(a => a).map(m => m.split(' ').filter(a => a).map(a => parseInt(a)))

let seedRanges = []
for(let i = 0; i < seeds.length; i = i + 2){
  seedRanges.push([seeds[i], seeds[i+1]])
}

let lowest = Number.MAX_SAFE_INTEGER

seedRanges.map(seedRange =>{
  for(let sr = seedRange[0]; sr < (seedRange[0] + seedRange[1]); sr++){
    let carry = sr
    for(let i = 1; i < sections.length; i++){
      const mappings = getMappings(sections[i])
      const map = mappings.find(m => ((m[1] <= carry) && (carry < (m[1] + m[2]))))
      if(map) {
        const diff = carry - map[1]
        carry = map[0] + diff
      }
    }
    lowest = Math.min(lowest, carry)
  }
})

console.log(lowest)


