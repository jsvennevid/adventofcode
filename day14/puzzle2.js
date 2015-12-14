var fs = require('fs');

fs.readFile('day14.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var reindeers = {}, states = {};
 inp.trim().split(/\n/g).forEach(function (line) {
  m = line.match(/(\w+) can fly ([0-9]+) km\/s for ([0-9]+) seconds, but then must rest for ([0-9]+) seconds./);
  if (m) {
   reindeers[m[1]] = m.slice(2, 5).map(function (v) { return Number(v); });
   states[m[1]] = { travel: Number(m[3]), wait: 0, distance: 0, score: 0 };
  }
 });

 for (var t = 0; t < 2503; ++t) {
  var leadReindeer = [], leadDistance = 0;
  for (var reindeer in reindeers) {
   var data = reindeers[reindeer];
   var state = states[reindeer];

   if (state.wait > 0) {
    state.wait -= 1;
	if (state.wait == 0) {
	 state.travel = data[1];
	}
   } else if (state.travel > 0) {
    state.travel -= 1;
	state.distance += data[0];
	if (state.travel == 0) {
	 state.wait = data[2];
	}
   }
   
   if (leadDistance < state.distance) {
    leadReindeer = [reindeer];
	leadDistance = state.distance;
   } else if (leadDistance == state.distance) {
    leadReindeer.push(reindeer);
   }
  }

  leadReindeer.forEach(function (reindeer) {
   states[reindeer].score += 1;
  });  
 }

 console.log(Object.keys(states).reduce(function (curr, reindeer) {
  return Math.max(curr, states[reindeer].score);
 }, 0));
});