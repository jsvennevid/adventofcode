var limit = 29000000, max = limit / 10;

var houses = new Array(1 + max);
for (var i = 1; i <= max; ++i) houses[i] = 0;

for (var i = 1; i <= max; ++i) {
    for (var j = i; j <= max; j += i) houses[j] += i * 10;
    if (houses[i] >= limit) break;
}
console.log(i);
