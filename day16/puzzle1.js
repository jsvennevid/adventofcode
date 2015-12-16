var fs = require('fs');

var criterias = {
 children: 3,
 cats: 7,
 samoyeds: 2,
 pomeranians: 3,
 akitas: 0,
 vizslas: 0,
 goldfish: 5,
 trees: 3,
 cars: 2,
 perfumes: 1
};

fs.readFile('day16.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var aunts = {};
 inp.trim().split(/\r?\n/g).forEach(function (line) {
  m = line.match(/(Sue [0-9]+): (.*)/);
  if (!m) return;

  var attrs = {}, t;
  var re = /([a-z]+): ([0-9]+),?/gi;
  while (t = re.exec(m[2])) attrs[t[1]] = Number(t[2]);

  aunts[m[1]] = attrs;
 });

 
 var keys = Object.keys(criterias);
 for (var i in aunts) {
  var aunt = aunts[i];
  
  var match = true;
  for (var j in criterias) {
   if (!aunt.hasOwnProperty(j)) {
    continue;
   }
   
   if (aunt[j] !== criterias[j]) {
    match = false;
	break;
   }
  }

  if (!match) continue;
  console.log(i, aunt);
 }
});