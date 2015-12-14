var fs = require('fs');

fs.readFile('day14.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var reindeers = {};
 inp.trim().split(/\n/g).forEach(function (line) {
  m = line.match(/(\w+) can fly ([0-9]+) km\/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds./);
  if (m) {
   reindeers[m[1]] = m.slice(2, 5).map(function (v) { return Number(v); });
  }
 });

 var time = 2503;
 
 var bestLength = 0;
 for (var reindeer in reindeers) {
  var data = reindeers[reindeer];
  var cycle = data[1] + data[2];
  var iterations = Math.floor(time / cycle);
  var remaining = Math.min(time - iterations * cycle, data[1]);
  var travel = (iterations * data[0] * data[1]) + data[0] * remaining;
  if (travel > bestLength) {
   bestLength = travel;
  }
 }
 console.log(bestLength);
});