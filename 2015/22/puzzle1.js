// [ cost, damage, heal, [ turns, damage, shield, recharge ] ]
var spells = [
    [ 53, 4, 0 ], // Magic Missile
    [ 73, 2, 2 ], // Drain
    [ 113, 0, 0, [ 6, 0, 7 ] ], // Shield
    [ 173, 0, 0, [ 6, 1, 3 ] ], // Poison
    [ 229, 0, 0, [ 5, 2, 101 ] ] // Recharge
];

function Effects(effects) {
    this.effects = effects ? effects.map(function (effect) { return effect.slice(0); }) : [];
}

Effects.prototype.clone = function () {
    return new Effects(this.effects);
};

Effects.prototype.tick = function (player, target) {
    var results = [0,0,0];
    this.effects = this.effects.map(function (effect) {
        results[effect[1]] = effect[2];
        effect[0] -= 1;
        return effect[0] > 0 ? effect : undefined;
    }).filter(function (effect) {
        return !!effect;
    });

    if (results[1] > 0) {
        target.hp -= results[1];
    }

    if (results[2] > 0) {
        player.mana += results[2];
    }

    return results[0];
};

Effects.prototype.has = function (curr) {
    return this.effects.filter(function (effect) {
        return effect[1] == curr[1];
    }).length > 0;
};

Effects.prototype.add = function (effect) {
    this.effects.push(effect.slice(0));
};

function Mob(name, hp,dam,mana,effects) {
    this.name = name;
    this.hp = hp;
    this.dam = dam;
    this.mana = mana;
    this.effects = effects;
}

Mob.prototype.clone = function () {
    return new Mob(this.name, this.hp, this.dam, this.mana, this.effects ? this.effects.clone() : undefined);
};

var player = new Mob("player", 50, 0, 500, new Effects());
var boss = new Mob("boss", 58, 9);

var best = Number.MAX_VALUE;

function round(player, boss, spell, cost) {
    player = player.clone();
    boss = boss.clone();

    player.mana -= spell[0];
    if (player.mana < 0) {
        return;
    }

    // effects

    player.effects.tick(player, boss);

    // cast

    boss.hp -= spell[1];
    player.hp += spell[2];

    if (spell[3]) {
        if (player.effects.has(spell[3])) {
            return;
        }

        player.effects.add(spell[3]);
    }

    // boss

    var armor = player.effects.tick(player, boss);

    if (boss.hp <= 0) {
        if (best > cost) {
            best = cost;
        }
        return;
    }

    player.hp -= Math.max(1, boss.dam - armor);

    if (player.hp <= 0) {
        return;
    }

    // next turn

    for (var spell of spells) {
        if (spell[0] + cost > best) continue;
        round(player, boss, spell, cost + spell[0]);
    }
}

for (var spell of spells) {
    round(player, boss, spell, spell[0]);
}
console.log(best);
