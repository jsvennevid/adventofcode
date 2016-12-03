var fs = require('fs');
var crypto = require('crypto');

var key = fs.readFileSync('day4.txt', 'utf-8').trim();

for (var i = 0;;++i) {
 var md5 = crypto.createHash('md5');
 var hash = md5.update(key + i.toString()).digest('hex');
 if (hash.indexOf('00000') == 0) break;
}
console.log(i);
