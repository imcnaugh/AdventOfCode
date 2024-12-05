import fs from 'fs'

const isTest = true;
const input = fs.readFileSync(`resources/day12${isTest ? '.test' : ''}.txt`, 'utf-8')

const data = input.split("\n");
const reps = 1; // IMPORTANT: change this constant to 5 to get the answer for part 2
let result = 0;

for (let line of data) {

    let record_small = line.split(" ")[0];
    let sig_small = [...line.matchAll(/\d+/g)].map(x => +(x[0]));
    let sig = [0];
    let record = "";

    for (let i = 0; i < reps; i++) {
        record = record + (record_small + "?");
        sig = sig.concat(sig_small);
    }

    let memo = [];
    for (let i = 0; i < record.length; i++) {
        memo[i] = [];
    }

    let countPaths = (rec_index, s_index) => {
        if (rec_index === -1 && s_index === 0) { return 1; }

        if (memo[rec_index]) { return memo[rec_index][s_index] ?? 0; }

        return 0;
    };

    for (let sig_index = 0; sig_index < sig.length; sig_index++) {
        for (let record_index = 0; record_index < record.length; record_index++) {
            let currentCount = 0;

            if (record[record_index] !== '#') {
                currentCount += countPaths(record_index - 1, sig_index);
            }

            if (sig_index > 0) {
                let doCount = true;

                for (let k = 1; k <= sig[sig_index]; k++) {
                    if (record[record_index - k] === '.') {
                        doCount = false;
                    }
                }

                if (record[record_index] === '#') {
                    doCount = false;
                }

                if (doCount) {
                    currentCount += countPaths(record_index - sig[sig_index] - 1, sig_index - 1);
                }
            }

            memo[record_index][sig_index] = currentCount;
        }
    }

    result += memo[record.length - 1][sig.length - 1];
}

console.log(`Part ${reps > 1 ? 2 : 1} ->`, result);