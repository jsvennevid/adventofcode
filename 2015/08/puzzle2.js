var fs = require('fs');

fs.readFile('day8.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var chars = 0, mem = 0;
 inp.trim().split(/\n/g).forEach(function (line) {
  chars += line.length;
  mem += line.replace(/(\"|\\)/gi, function (a) { return "\\" + a; }).length + 2;
 });
 console.log(mem - chars);
});
