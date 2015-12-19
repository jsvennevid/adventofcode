var fs = require('fs');

fs.readFile('day19.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    inp = inp.trim().split(/\r?\n/g);
    var initial = inp.splice(-2, 2).slice(1)[0];
    var transforms = {};
    inp.forEach(function (line) {
        var m = line.match(/(\w+) => (\w+)/);
        if (m) {
            transforms[m[2]] = m[1];
        }
    });

    var curr = initial;
    var targets = Object.keys(transforms).sort(function (a,b) {
        return b.length - a.length;
    });

    var steps = 0;
    do {
        for (var i in targets) {
            var target = targets[i];

            var n = curr.indexOf(target);
            if (n >= 0) {
                var source = transforms[target];
                curr = curr.slice(0,n) + source + curr.slice(n + target.length);
                ++ steps;
                break;
            }
        }
    } while (curr != "e");
    console.log(steps);
});
