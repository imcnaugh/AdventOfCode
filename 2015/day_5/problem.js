import fs from 'fs';

const input = fs.readFileSync('./input.txt', 'utf-8');

const vowels = ['a', 'e', 'i', 'o', 'u'];
const bad_combos = [
    ['a', 'b'],
    ['c', 'd'],
    ['p', 'q'],
    ['x', 'y'],
];

function is_nice_part_1(line) {
    let previous = line[0]

    let vowel_count = vowels.includes(previous) ? 1 : 0;
    let is_a_repate = false;

    for (let i = 1; i < line.length; i++) {
        let current = line[i];

        let contains_bad_combo = bad_combos.filter((combo) => combo[0] === previous && combo[1] === current).length;
        if (contains_bad_combo) {
            return false;
        }

        if (current === previous) {
            is_a_repate = true;
        }
        if (vowels.includes(current)) {
            vowel_count++;
        }
        previous = current;
    }
    return is_a_repate && vowel_count >= 3;
}

// console.log(is_nice_part_1("xy"));

function is_nice_part_2(line) {
    let pair_seen_at = new Map();
    let has_pair = false;
    let odd_repeat = false;

    for (let i = 0; i < line.length - 1; i++) {
        let pair = line[i] + line[i + 1];

        if (pair_seen_at.has(pair)) {
            if (i >= pair_seen_at.get(pair) + 2) {
                has_pair = true;
            }
        } else {
            pair_seen_at.set(pair, i);
        }

        if (i < line.length - 2 && line[i] === line[i + 2]) {
            odd_repeat = true;
        }
    }

    return has_pair && odd_repeat;
}

let total = input.split('\n').filter((line) => {
    return is_nice_part_2(line);
}).length;

console.log(total);

console.log(is_nice_part_2("xyxyx"));