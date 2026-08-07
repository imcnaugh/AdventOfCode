import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

class Comput {
    constructor(inst) {
        this.a = 0
        this.b = 0
        this.c = 0
        this.d = 0
        this.inst = inst
        this.currentInst = 0
    }

    cpy(x, y){
        let val= null
        switch(x){
            case "a":
                val = this.a
                break
            case "b":
                val = this.b
                break
            case "c":
                val = this.c
                break
            case "d":
                val = this.d
                break
            default:
                val = parseInt(x)
        }
        switch(y){
            case "a":
                this.a = val
                break
            case "b":
                this.b = val
                break
            case "c":
                this.c = val
                break
            case "d":
                this.d = val
                break
        }
        this.currentInst++
    }

    inc(x) {
        switch (x) {
            case "a":
                this.a++
                break
            case "b":
                this.b++
                break
            case "c":
                this.c++
                break
            case "d":
                this.d++
                break
        }
        this.currentInst++
    }
    dec(x) {
        switch (x) {
            case "a":
                this.a--
                break
            case "b":
                this.b--
                break
            case "c":
                this.c--
                break
            case "d":
                this.d--
                break
        }
        this.currentInst++
    }

    jnz(x, y) {
        if(x === "a" && this.a === 0){
            this.currentInst++
            return
        }
        if(x === "b" && this.b === 0){
            this.currentInst++
            return
        }
        if(x === "c" && this.c === 0){
            this.currentInst++
            return
        }
        if(x === "d" && this.d === 0){
            this.currentInst++
            return
        }
        this.currentInst += parseInt(y)
    }

    run(){
        while(this.currentInst < this.inst.length){
            const curInst = this.inst[this.currentInst]
            switch(curInst[0]){
                case "cpy":
                    this.cpy(curInst[1], curInst[2])
                    break
                case "inc":
                    this.inc(curInst[1])
                    break
                case "dec":
                    this.dec(curInst[1])
                    break
                case "jnz":
                    this.jnz(curInst[1], curInst[2])
                    break
            }
        }
    }
}

let instPart1 = input.split('\n').map(x => x.split(' '))

let comp = new Comput(instPart1)
comp.run()

console.log(`Part 1: ${comp.a}`)