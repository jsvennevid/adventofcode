var fs = require('fs');

var width = 100, height = 100, iter = 100;

function count(input, x0, y0, x1, y1, x, y) {
    var num = 0;
    for (var j = y0; j < y1; ++j) {
        for (var i = x0; i < x1; ++i) {
            if ((i == x) && (j == y)) continue;
            var address = i + j * (width + 2);
            if ('#' == input[address]) ++num;
        }
    }
    return num;
}

function life(input) {
    var output = new Array((width + 2) * (height + 2));
    for (var y = 1; y <= height; ++y) {
        for (var x = 1; x <= width; ++x) {
            var address = x + y * (width + 2);
            var on = count(input, x - 1, y - 1, x + 2, y + 2, x, y);
            output[address] = input[address] == '#' ? (on == 2 || on == 3) ? '#' : '.' : on == 3 ? '#' : '.';
        }
    }
    return output;
}

function print(input) {
    for (var y = 1; y <= height; ++y) {
        var s = "";
        for (var x = 1; x <= width; ++x) {
            s += input[x + y * (width + 2)]
        }

        console.log(s);
    }
}

fs.readFile('day18.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    var grid = new Array((width + 2) * (height + 2));
    inp.trim().split(/\r?\n/g).forEach(function (line,y) {
        line.split('').forEach(function (c, x) {
            grid[1 + x + (1 + y) * (width + 2)] = c;
        });
    });

    for (var i = 0; i < iter; ++i) {
        grid = life(grid);
    }
    print(grid);
    console.log(count(grid,1,1,101,101));
});