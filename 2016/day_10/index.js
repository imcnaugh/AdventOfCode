import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')
// const input = "value 5 goes to bot 2\n" +
//     "bot 2 gives low to bot 1 and high to bot 0\n" +
//     "value 3 goes to bot 1\n" +
//     "bot 1 gives low to output 1 and high to bot 0\n" +
//     "bot 0 gives low to output 2 and high to output 0\n" +
//     "value 2 goes to bot 2"

const items = new Map()

class Bot {
    constructor(id){
        this.id = id
        this.instructions = []
        this.items = []
    }

    addInstruction(inst) {
        this.instructions.push(inst)

        if(this.items.length === 2){
            this.executeNextInstruction()
        }
    }

    executeNextInstruction(){
        if(this.instructions.length > 0) {
            const toProcess = this.instructions.pop()
            const {lowReciver, highReciver} = toProcess()
            items.get(highReciver).giveItem(this.items.pop())
            items.get(lowReciver).giveItem(this.items.pop())
        }
    }

    giveItem(item){
        this.items.push(item)
        this.items.sort((a, b) => a - b)

        if(this.items.length === 2){
            if(this.items[0] === 17 && this.items[1] === 61)
                console.log(`part 1: ${this.id}`)

            this.executeNextInstruction()
        }
    }
}

const giveRx = /(.*) gives low to (.*) and high to (.*)/
const valueRx = /value (\d+) goes to (.*)/

input.split('\n').forEach(line => {
    if(line.startsWith("bot")) {
        const match = line.match(giveRx)
        const giveBot = match[1]
        const lowReciver = match[2]
        const highReciver = match[3]

        if(!items.has(giveBot)) items.set(giveBot, new Bot(giveBot))
        if(!items.has(lowReciver)) items.set(lowReciver, new Bot(lowReciver))
        if(!items.has(highReciver)) items.set(highReciver, new Bot(highReciver))

        const inst = () => {
            return {lowReciver, highReciver}
        };
        items.get(giveBot).addInstruction(inst)

    } else if(line.startsWith("value")) {
        const match = line.match(valueRx)
        const value = parseInt(match[1])
        const giveBot = match[2]

        if(!items.has(giveBot)) items.set(giveBot, new Bot(giveBot))
        items.get(giveBot).giveItem(value)
    }
})

console.log("idk")


