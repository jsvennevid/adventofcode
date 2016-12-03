var fs = require('fs');

function* permute(initial, transforms) {
    var history = {};

    var sources = Object.keys(transforms);
    for (var i = 0; i < initial.length; ++i) {
        for (var j = 0; j < sources.length; ++j) {
            var source = sources[j];
            if (initial.slice(i, i + source.length) == source) {
                var targets = transforms[source];
                for (var k = 0; k < targets.length; ++k) {
                    var target = targets[k];
                    var result = initial.slice(0,i) + target + initial.slice(i + source.length);
                    if (!history.hasOwnProperty(result)) {
                        history[result] = true;
                        yield result;
                    }
                }
            }
        }
    }
}

fs.readFile('day19.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    inp = inp.trim().split(/\r?\n/g);
    var initial = inp.splice(-2, 2).slice(1)[0];
    var transforms = {};
    inp.forEach(function (line) {
        var m = line.match(/(\w+) => (\w+)/);
        if (m) {
            transforms[m[1]] = (transforms[m[1]] || []);
            transforms[m[1]].push(m[2]);
        }
    });

    var result = 0;
    for (var molecule of permute(initial, transforms)) {
        ++ result;
    }
    console.log(result);
});