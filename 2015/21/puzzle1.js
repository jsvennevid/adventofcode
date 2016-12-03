var weapons = [
    [8,4,0], // Dagger
    [10,5,0], // Shortsword
    [25,6,0], // Warhammer
    [40,7,0], // Longsword
    [74,8,0] // Greataxe
];

var armors = [
    [13,0,1], // Leather
    [31,0,2], // Chainmail
    [53,0,3], // Splintmail
    [75,0,4], // Bandedmail
    [102,0,5] // Platemail
];

var rings = [
    [25,1,0], // Damage +1
    [50,2,0], // Damage +2
    [100,3,0], // Damage +3
    [20,0,1], // Defense +1
    [40,0,2], // Defense +2
    [80,0,3] // Defense +3
];

function fight(player, boss) {
    player = player.slice(0);
    boss = boss.slice(0);

    var mobs = [player,boss];

    while (mobs.filter(function (mob) { return mob[0] == 0; }).length == 0) {
        for (var i = 0; (i < mobs.length) && mobs[i][0] > 0; ++i) {
            var att = mobs[i];
            var def = mobs[i^1];

            var damage = Math.max(1, att[1] - def[2]);

            def[0] = Math.max(0, def[0] - damage);
        }
    }

    return player[0] > 0;
}

function* permute(items, min, max) {
    if (min < 1) {
        yield [];
        min = 1;
    }

    for (var i = 0; i < items.length; ++i) {
        var subItems = items.slice(0,i).concat(items.slice(i+1));
        yield [items[i]];

        if (min < max) {
            for (var item of permute(subItems, min + 1, max)) {
                yield [items[i]].concat(item);
            }
        }
    }
}

function create(hp, inventory) {
    return [hp].concat(inventory.reduce(function (curr, item) {
        return [curr[0] + item[1], curr[1] + item[2], curr[2] + item[0]];
    }, [0,0,0]));
}

var boss = [103,9,2];
var best = Number.MAX_VALUE;
for (var weapon of permute(weapons, 1, 1)) {
    for (var armor of permute(armors, 0, 1)) {
        for (var ring of permute(rings, 0, 2)) {
            var player = create(100, weapon.concat(armor).concat(ring));
            var cost = player[3];
            if (fight(player, boss) && (cost < best)) {
                best = cost;
            }
        }
    }
}
console.log(best);
