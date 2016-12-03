var fs = require('fs');

var wires = {};
var ops = {
 'NOT': function (a,b,results) { return ~visit(b,results) & 65535; },
 'AND': function (a,b,results) { return visit(a,results) & visit(b,results); },
 'OR': function (a,b,results) { return visit(a,results) | visit(b,results); },
 'RSHIFT': function (a,b,results) { return visit(a,results) >> visit(b,results); },
 'LSHIFT': function (a,b,results) { return (visit(a,results) << visit(b,results)) & 65535; },
 undefined: function (a,b,results) { return b.match(/^[0-9]+$/) ? Number(b) : visit(b,results); }
}

function visit(wire, results) {
 if (results[wire]) {
  return results[wire];
 }

 if (wire.match(/^[0-9]+$/)) {
  return Number(wire);
 }

 var data = wires[wire];
 return (results[wire] = ops[data[0]](data[1],data[2],results));
}

fs.readFile('day7.txt', 'utf-8', function (err, inp) {
 inp.trim().split(/\n/g).forEach(function (line) {
  var m = line.match(/^(?:([a-z0-9]+) )?(?:(NOT|AND|OR|LSHIFT|RSHIFT) )?([a-z0-9]+) -> ([a-z]+)$/);
  if (m) {
   wires[m[4]] = [m[2], m[1], m[3]];
  }
 });

 console.log("a: %d", visit("a", {}));
});
