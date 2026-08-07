import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

const containsRx = /contains a (.*)/

let itemsOnFloor = input.split('\n').map(line => {
    let containsMap = line.match(containsRx)
    if(containsMap) {
        return containsMap[1].split(',').map(i => i.trim()).map(i => {
            let words = i.split(' ')
            return `${words[0].charAt(0)}-${words[words.length - 1].charAt(0)}`
        })
    } else {
        return []
    }
})

let currentFloor = 0
let currentlyInElevator = []

let moveCount = 0

function moveFromFloorToElevator(item) {
    if(!itemsOnFloor[currentFloor].includes(item)) {
        throw new Error(`item ${item} not on floor ${currentFloor}`)
    }
    if(currentlyInElevator.length >= 2) {
        throw new Error(`Elevator full`)
    }

    const index = itemsOnFloor[currentFloor].indexOf(item)
    itemsOnFloor[currentFloor].splice(index, 1)
    currentlyInElevator.push(item)
}

function moveFromElevatorToFloor(item) {
    if(!currentlyInElevator.includes(item)) {
        throw new Error(`item ${item} not in elevator`)
    }
    const index = currentlyInElevator.indexOf(item)
    currentlyInElevator.splice(index, 1)
    itemsOnFloor[currentFloor].push(item)
}

function moveElevatorToFloor(newFloor){
    if(newFloor < 0 || newFloor > 3) {
        throw new Error(`invalid floor ${newFloor}`)
    }
    if(currentlyInElevator.length <= 0) {
        throw new Error(`no items in elevator`)
    }
    moveCount += Math.abs(newFloor - currentFloor)
    currentFloor = newFloor
}

function print() {
    for(let i = 3; i >= 0; i--) {
        let elevatorString = ""
        if(i === currentFloor) {
            elevatorString = `Elevator: ${currentlyInElevator.join(', ')}`
        }

        console.log(`Floor ${i}: ${itemsOnFloor[i].join(', ')} ${elevatorString}`)
    }
}


moveFromFloorToElevator('t-g')
moveFromFloorToElevator('k-g')
moveElevatorToFloor(3)
moveFromElevatorToFloor('t-g')
moveElevatorToFloor(0)
moveFromFloorToElevator('s-g')
moveElevatorToFloor(3)
moveFromElevatorToFloor('s-g')
moveElevatorToFloor(2)
moveFromFloorToElevator('p-g')
moveElevatorToFloor(3)
moveFromElevatorToFloor('p-g')
moveElevatorToFloor(2)
moveFromFloorToElevator('r-g')
moveElevatorToFloor(3)
moveFromElevatorToFloor('r-g')
moveElevatorToFloor(1)
moveFromFloorToElevator('k-m')
moveElevatorToFloor(3)
moveFromElevatorToFloor('k-m')
moveFromElevatorToFloor('k-g')
moveFromFloorToElevator('t-g')
moveElevatorToFloor(0)
moveFromFloorToElevator('t-m')
moveElevatorToFloor(3)
moveFromElevatorToFloor('t-m')
moveFromElevatorToFloor('t-g')
moveFromFloorToElevator('s-g')
moveElevatorToFloor(1)
moveFromFloorToElevator('s-m')
moveElevatorToFloor(3)
moveFromElevatorToFloor('s-m')
moveFromElevatorToFloor('s-g')
moveFromFloorToElevator('r-g')
moveElevatorToFloor(2)
moveFromFloorToElevator('r-m')
moveElevatorToFloor(3)
moveFromElevatorToFloor('r-g')
moveFromElevatorToFloor('r-m')
moveFromFloorToElevator('p-g')
moveElevatorToFloor(2)
moveFromFloorToElevator('p-m')
moveElevatorToFloor(3)

print()

console.log(`Part 1: ${moveCount}`)