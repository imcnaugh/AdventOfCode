import fs from 'node:fs'

const input = fs.readFileSync('input.txt', 'utf8')

const Direction = Object.freeze({
    north: 0,
    south: 1,
    east: 2,
    west: 3
})

const TurnDirection = Object.freeze({
    R: 0,
    L: 1
})

function getNewDir(d, turn) {
    switch(d) {
        case Direction.north:
            return turn === TurnDirection.R ? Direction.east : Direction.west;
        case Direction.east:
            return turn === TurnDirection.R ? Direction.south : Direction.north;
        case Direction.south:
            return turn === TurnDirection.R ? Direction.west : Direction.east;
        case Direction.west:
            return turn === TurnDirection.R ? Direction.north : Direction.south;
    }
}

const directions = input.split(',').map(s => s.trim()).map(s => {
    const turnDir = s[0] === 'R' ? TurnDirection.R : TurnDirection.L;
    const dist = s.slice(1);
    return {
        turnDir,
        dist: Number.parseInt(dist)
    }
}).reduce((acc, dir) => {
    acc.d = getNewDir(acc.d, dir.turnDir);
    switch(acc.d){
        case Direction.north:
            acc.y += dir.dist;
            break;
        case Direction.east:
            acc.x += dir.dist;
            break;
        case Direction.south:
            acc.y -= dir.dist;
            break;
        case Direction.west:
            acc.x -= dir.dist;
            break;
    }
    return (acc)
}, {d: Direction.north, x: 0, y: 0})

const blocksAway = directions.x + directions.y
console.log(blocksAway)