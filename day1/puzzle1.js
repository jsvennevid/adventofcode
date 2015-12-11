var fs = require('fs');

var vals = {'(': 1, ')': -1};

fs.readFile('day1.txt', 'utf-8', function (err, inp) {
 if (err) throw err;
 inp = inp.trim().split("");

 for (var i = 0, c = 0; i < inp.length; ++i) {
  c += vals[inp[i]];
 }
 console.log(c);
});
