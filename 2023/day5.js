import fs from 'fs'

const isTest = false
const input = fs.readFileSync(`resources/day5${isTest? '.test' : ''}.txt`, 'utf-8')

const sections = input.split('\n\n')

const seeds = sections[0].split(':')[1].split(' ').filter(a => a).map(a => parseInt(a))
const getMappings = (line) => line.split(':')[1].split('\n').filter(a => a).map(m => m.split(' ').filter(a => a).map(a => parseInt(a)))



const locations = seeds.map(seed => {
  let carry = seed
  for(let i = 1; i < sections.length; i++){
    const mappings = getMappings(sections[i])
    const map = mappings.find(m => ((m[1] <= carry) && (carry < (m[1] + m[2]))))
    if(map) {
      const diff = carry - map[1]
      carry = map[0] + diff
    }
  }
  return carry
})

let lowest = locations[0]
for(const l of locations){
  if(l < lowest) lowest = l
}

console.log(lowest)