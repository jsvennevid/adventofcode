var fs = require('fs');

var ops = {
    'hlf': function (r) { regs[r] >>= 1; },
    'tpl': function (r) { regs[r] *= 3; },
    'inc': function (r) { ++regs[r]; },
    'jmp': function (ofs) { pc += ofs-1; },
    'jie': function (r, ofs) { if ((regs[r]&1) == 0) pc += ofs-1; },
    'jio': function (r, ofs) { if (regs[r] == 1) pc += ofs-1; }
};

var regs = { 'a': 0, 'b': 0 };
var pc = 0;

fs.readFile('day23.txt', 'utf-8', function (err, inp) {
    if (err) throw err;

    var ins = inp.trim().split(/\r?\n/g).map(function (line) {
        var m = line.match(/(hlf|tpl|inc|jmp|jie|jio) ([^,]+)(?:, (.+))?/i);
        return m ? m.slice(1,4) : undefined;
    }).filter(function (n) { return !!n });

    while (pc >= 0 && pc < ins.length) {
        var n = ins[pc++];
        ops[n[0]].apply(null, n.slice(1));
    }

    console.log(regs.b);
});
