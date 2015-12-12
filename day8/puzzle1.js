var fs = require('fs');

fs.readFile('day8.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var chars = 0, mem = 0;
 inp.trim().split(/\n/g).forEach(function (line) {
  var d = line.match(/^\"(.*)\"$/,line)
  if (!d) {
   return;
  }
  chars += line.length;
  var re = /(\\\\|\\"|\\x([0-9a-f]{2})|(.+?))/gi;
  var s = d[1], m;
  while (m = re.exec(s)) {
   mem += m[1][0] == '\\' ? 1 : m[1].length;
  }
 });
 console.log(chars - mem);
});
