var fs = require('fs');

var permute = function *(ingredients) {
 var state = new Array(ingredients.length - 1);
 for (var i = 0; i < state.length; ++i) state[i] = 0;
 
 while (true) {
  for (var i = 0; i < state.length; ++i) {
   state[i]++;
   if (state.reduce(function (curr, v) { return curr + v; }) < 100) break;
   if (state[state.length-1] == 100) return;
   state[i] = 0;
  }

  var curr = state.concat(100 - state.reduce(function (curr, v) { return curr + v}));
  yield curr;
 }
}

function getScore(ingredients, state) {
  return ingredients[0].reduce(function (curr,v,i) {
   var sum = Math.max(0, ingredients.reduce(function (curr,v,j) { return curr + v[i] * state[j]; }, 0));
   return curr == undefined ? sum : sum * curr;
  }, undefined);
}

fs.readFile('day15.txt', 'utf-8', function (err, inp) {
 if (err) throw err;
 
 var ingredients = [];
 inp.trim().split(/\r?\n/g).forEach(function (line) {
  m = line.match(/(\w+): capacity ([^,]+), durability ([^,]+), flavor ([^,]+), texture ([^,]+), calories (.+)/)
  if (m) {
   ingredients.push([Number(m[2]), Number(m[3]), Number(m[4]), Number(m[5])]);
  }
 });

 var bestScore = 0, curr = 0;
 for (var p = permute(ingredients), state = p.next(); !state.done; state = p.next()) {
  bestScore = Math.max(bestScore, getScore(ingredients, state.value));
 }
 console.log(bestScore); 
});