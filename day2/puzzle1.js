var fs = require('fs');

fs.readFile('day2.txt', 'utf-8', function (err, inp) {
 if (err) throw err;
 console.log(inp.trim().split(/\n/g).reduce(function (curr, line) {
  var m = line.match(/([0-9]+)x([0-9]+)x([0-9]+)/);
  var l = Number(m[1]), w = Number(m[2]), h = Number(m[3]);
  var area = (2*l*w)+(2*w*h)+(2*h*l), extra = Math.min(l*w,w*h,h*l);
  return curr + area + extra;
 }, 0));
});
