import fs from 'fs';

let input = fs.readFileSync('input.txt', 'utf8').trim();

let origins = input.split('\n').reduce((acc, line) => {
    var parts = line.split(' -> ');
    acc.set(parts[1], parts[0]);
    return acc;
}, new Map());

// A cache to store calculated values
const cache = new Map();

const getValue = (key) => {
    // If we've already calculated this wire, return it immediately
    if (cache.has(key)) return cache.get(key);

    // If the "key" is actually a number (like in "1 AND r"), return the number
    if (!isNaN(parseInt(key))) return parseInt(key) & 0xFFFF;

    let origin = origins.get(key);
    let result;

    let parts = origin.split(' ');

    if (parts.length === 1) {
        // Handle: 123 -> x  OR  y -> x
        let val = parts[0];
        result = isNaN(parseInt(val)) ? getValue(val) : parseInt(val);
    } else if (parts[0] === 'NOT') {
        // Handle: NOT e -> f
        result = ~getValue(parts[1]);
    } else {
        var op = parts[1];
        var left = parts[0];
        var right = parts[2];

        if (op === 'AND') {
            result = getValue(left) & getValue(right);
        } else if (op === 'OR') {
            result = getValue(left) | getValue(right);
        } else if (op === 'LSHIFT') {
            result = getValue(left) << parseInt(right);
        } else if (op === 'RSHIFT') {
            result = getValue(left) >> parseInt(right);
        }
    }

    // Apply 16-bit mask and store in cache
    result = (result & 0xFFFF);
    cache.set(key, result);
    return result;
}

console.log(getValue("a"));