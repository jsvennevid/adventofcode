var fs = require('fs');

function visit(root) {
 if (Array.isArray(root)) {
  return root.reduce(function (curr, val) {
   return curr + visit(val);
  }, 0);
 } else if (typeof root == "object") {
  var keys = Object.keys(root);
  if (keys.filter(function (key) {
   return root[key] === "red";
  }).length > 0) {
   return 0;
  }
  return keys.reduce(function (curr, key) {
   return curr + visit(root[key]);
  }, 0);
 } else if (typeof root == "number") {
  return root;
 } else {
  return 0;
 }
}

fs.readFile('day12.txt', 'utf-8', function (err, inp) {
 if (err) throw err;

 var doc = JSON.parse(inp);
 console.log(visit(doc));
});
