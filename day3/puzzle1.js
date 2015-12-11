var fs = require('fs');

fs.readFile('day3.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var x = 0, y = 0, houses = {};
 var dirs = { '^': [0,-1], 'v': [0,1], '>': [1,0], '<': [-1,0] };
 inp.trim().split("").forEach(function (c) {
  var address = x + ":" + y;
  houses[address] = (houses[address] || 0) + 1;
  var dir = dirs[c];
  x += dir[0]; y += dir[1];
 });
 console.log(Object.keys(houses).length);
});
