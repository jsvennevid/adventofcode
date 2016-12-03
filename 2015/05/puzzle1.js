var fs = require('fs');

function count(re, s) {
 for (var i = 0; m = re.exec(s); ++i);
 return i;
}

fs.readFile('day5.txt', 'utf-8', function (err, inp) {
 if (err) throw err;
 console.log(inp.trim().split(/\n/g).reduce(function (curr, line) {
  var ok = (count(/[aeiou]/g,line) >= 3 && count(/(.)\1/g,line) > 0 && count(/(ab|cd|pq|xy)/g,line) == 0)
  return curr + (ok ? 1 : 0);
 }, 0));
});
