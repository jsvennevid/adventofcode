var fs = require('fs');

var ops = {
 'turn on': function () { return 1; },
 'turn off': function () { return 0; },
 'toggle': function (v) { return v^1; }
};

grid = new Array(1000 * 1000);

fs.readFile('day6.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 inp.trim().split(/\n/g).forEach(function (line) {
  var m = line.match(/(turn on|turn off|toggle) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)/);
  if (m) {
   for (var y = Number(m[3]), yn = Number(m[5]); y <= yn; ++y) {
    for (var x = Number(m[2]), xn = Number(m[4]); x <= xn; ++x) {
     grid[y * 1000 + x] = ops[m[1]](grid[y * 1000 + x] || 0); 
    }
   }
  }
 });

 console.log(grid.filter(function (v) {
  return !!v;
 }).length);
});

