var limit = 29000000, max = limit / 10;

var houses = new Array(1 + max);
for (var i = 1; i <= max; ++i) houses[i] = 0;

for (var i = 1; i <= max; ++i) {
    for (var j = i, n = 0; j <= max && n < 50; j += i, ++n) houses[j] += i * 11;
    if (houses[i] >= limit) break;
}
console.log(i);
