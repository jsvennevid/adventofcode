var fs = require('fs');

function* combinations(iterable, r) {
    var pool = iterable;
    var n = iterable.length;
    if (r > n) return;

    var indices = Array.apply(null, {length: r}).map(function (n,i) {return i;});
    yield indices.map(function (i) { return pool[i]; });
    while (true) {
        var found = false;
        for (var i = r - 1; i >= 0; --i){
            if (indices[i] != i + n - r) {
                found = true;
                break;
            }
        }
        if (!found) {
            break;
        }

        indices[i] += 1;
        for (var j = i + 1; j < r; ++j) {
            indices[j] = indices[j - 1] + 1
        }
        yield indices.map(function (i) { return pool[i]; });
    }
}

function sum(iterable) {
    return iterable.reduce(function (c,n) { return c + n; }, 0);
}

function product(iterable) {
    return iterable.reduce(function (c,n) { return c * n; });
}

function without(iterable, remove) {
    var out = []
    for (var v of iterable) {
        if (remove.indexOf(v) < 0) out.push(v);
    }
    return out;
}

function partition(packages, depth, max, weight) {
    if (depth == max) return weight == sum(packages);

    for (var i = 1; i < packages.length; ++i) {
        for (var x of combinations(packages, i)) {
            if (weight != sum(x)) continue;
            if (depth < 2 && !partition(without(packages, x), depth + 1, max, weight)) {
                continue;
            }
            return depth == 0 ? product(x) : true;
        }
    }
    return false;
}

fs.readFile('day24.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    var packages = inp.trim().split(/\r?\n/g).map(function (n) {
        return Number(n);
    });
    packages.sort(function (a,b) { return a - b; });

    console.log(partition(packages, 0, 4 - 1, sum(packages) / 4))
});