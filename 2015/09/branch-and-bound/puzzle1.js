var fs = require('fs');
var path = require('path');

function search(routes, compare) {
 var places = Object.keys(routes);

 var path = function (source, history, cost) {
  history.push(source);
  if (history.length == places.length) {
   this.route = history;
   this.cost = cost;
  } else {
   var destinations = Object.keys(routes[source]).filter(function (dest) {
    return history.indexOf(dest) < 0;
   }).map(function (dest) {
    return [routes[source][dest],dest];
   });
   destinations.forEach(function (dest) {
    if (!compare(cost + dest[0], this.cost)) return;
    path.call(this, dest[1], history.slice(0), cost + dest[0]);
   }.bind(this));
  }
 };

 this.route = [];

 if (compare(0,Number.MAX_VALUE)) {
  this.cost = Number.MAX_VALUE;
 } else {
  this.cost = 0;
 }

 places.forEach(function (source) {
  path.call(this, source, [], 0);
 }.bind(this));

 return this.optimalCost;
}

fs.readFile(path.join(__dirname, '../day9.txt'), 'utf-8', function (err, inp) {
 if (err) throw err;

 var routes = {};
 inp.trim().split(/\n/g).forEach(function (route) {
  m = route.match(/(\w+) to (\w+) = ([0-9]+)/i);
  if (!m) return;
  (routes[m[1]] = routes[m[1]] || {})[m[2]] = Number(m[3]); 
  (routes[m[2]] = routes[m[2]] || {})[m[1]] = Number(m[3]); 
 });

 console.log(new search(routes, function (x,y) { return x < y; }));
});

