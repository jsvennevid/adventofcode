var fs = require('fs');

function permute(current, remaining, results) {
 if (remaining.length == 0) {
  results.push(current);
  return results;
 }

 for (var i = 0; i < remaining.length; ++i) {
  var curr = remaining.slice(0);
  permute(current.concat(curr.splice(i,1)), curr, results);
 }
 
 return results;
}

fs.readFile('day13.txt', 'utf-8', function (err, inp) {

 var attendees = {'Myself': {}};
 inp.trim().split(/\n/g).forEach(function (line) {
 m = line.match(/(\w+) would (\w+) ([0-9]+) happiness units by sitting next to (\w+)./);
  if (m) {
   (attendees[m[1]] = attendees[m[1]] || {})[m[4]] = Number(m[3]) * (m[2] == 'gain' ? 1 : -1);
  }
 });

 var bestSeating = [], bestScore = Number.MIN_VALUE;
 permute([], Object.keys(attendees), []).forEach(function (seating) {
  var score = 0;
  for (var i = 0; i < seating.length; ++i) {
   var left = seating[((i-1) % seating.length + seating.length) % seating.length], right = seating[(i + 1) % seating.length];
   score += (attendees[seating[i]][left]||0) + (attendees[seating[i]][right]||0);
  }

  if (score > bestScore) {
   bestSeating = seating;
   bestScore = score;
  }
 });
 console.log(bestScore, bestSeating);
});