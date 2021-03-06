var fs = require('fs');

function* combinations(items) {
    for (var i = 0, n = Math.pow(2, items.length); i < n; ++i) {
        yield items.filter(function (item, j) {
            return (i & (1 << j)) > 0;
        });
    }
}

fs.readFile('day17.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    var sizes = inp.trim().split(/\r?\n/g).map(function (size) {
        return Number(size);
    });

    var count = 0, size = Number.MAX_VALUE;
    for (var combination of combinations(sizes)) {
        if (combination.reduce(function (curr, v) { return curr + v}, 0) != 150) {
            continue;
        }
        if (combination.length > size) {
            continue;
        } else if (combination.length == size) {
            ++ count;
            continue;
        }
        size = combination.length;
        count = 1;
    }
    console.log(count);
});