/* global HTML, m */

'use strict';

const $ = HTML({
    h: m,
    textConvert: (a) => `${a}`,
    combineId: true,
    combineClasses: true,
});

const STEP = 1500;
const UNIT_PAD = 2;
const BOARD_SIZE = 61;
const DEFAULT_AI = SimpleAI;
const STARTING_RESOURCES = 10;
const RESOURCE_GAIN_PER_STEP = 2;
const TOTAL_PLAYERS = 2;
const BASE_STATS = {
    movement: 0,
    range: 0,
    firepower: 0,
    health: 40,
    priority: -999,
};
const UNIT_STAT_BOUNDS = {
    movement: [0.25, 2, 0.25],
    range: [0, 12, 1],
    firepower: [0, 4, 1],
    health: [0, 10, 1],
};
const UNIT_PROPERTY_NAMES = {
    // Controller: ['controller', 'id'],
    Health: ['health'],
    // 'Max Health': ['stats', 'health'],
    Movement: ['stats', 'movement'],
    Range: ['stats', 'range'],
    Firepower: ['stats', 'firepower'],
};
const KEYBINDS = {
    a: ['health', 'Max Health'],
    s: ['movement', 'Movement'],
    d: ['range', 'Range'],
    f: ['firepower', 'Firepower'],
    v: [(s, p) => p.unit(s), 'Create'],
};

const board = Array(BOARD_SIZE).fill(null);
const units = new Set();
const players = [player(0), player(1, RESOURCE_GAIN_PER_STEP * 2)];
let alivePlayers = TOTAL_PLAYERS;
const handles = [TextPlayerHandle(players[0]), EmptyHandle(players[1])];

/// Utils

function minmax(a, min, max) {
    return Math.max(min, Math.min(a, max));
}

function bump(position, _direction = 0, cont = true) {
    // Returns the next available spot in the direction specified, or undefined if there aren't any.
    const direction = Math.sign(Math.sign(_direction) + Math.random() - 1 / 2);
    let res = position;
    while (board[res] !== undefined && board[res] !== null) {
        res += direction;
    }
    if (board[res] === undefined && cont) return bump(position, -direction, false);
    return res;
}

function getDeep(obj, path) {
    let res = obj;
    for (const a of path) {
        res = res[a];
    }
    return res;
}

function withinRange(unit) {
    const range = unit.stats.range;
    const pos = unit.position;
    const res = [];
    for (let i = pos - range; i <= pos + range; i++) {
        const b = board[i];
        if (b != null && b.controller !== unit.controller) {
            res.push(b);
        }
    }
    return res;
}

function computePrice(stats) {
    return stats.movement * 4 + ((Math.pow(stats.range, 1.2) + 1) * stats.firepower) / 2 + stats.health / 3;
}

function rainbow(numOfSteps, step) {
    // This function generates vibrant, "evenly spaced" colours (i.e. no clustering).
    // This is ideal for creating easily distinguishable vibrant markers in Google Maps and other apps.
    // Adam Cole, 2011-Sept-14
    // HSV to RBG adapted from:
    // http://mjijackson.com/2008/02/rgb-to-hsl-and-rgb-to-hsv-color-model-conversion-algorithms-in-javascript
    let r, g, b;
    const h = step / numOfSteps;
    const i = ~~(h * 6);
    const f = h * 6 - i;
    const q = 1 - f;
    switch (i % 6) {
        case 0:
            r = 1;
            g = f;
            b = 0;
            break;
        case 1:
            r = q;
            g = 1;
            b = 0;
            break;
        case 2:
            r = 0;
            g = 1;
            b = f;
            break;
        case 3:
            r = 0;
            g = q;
            b = 1;
            break;
        case 4:
            r = f;
            g = 0;
            b = 1;
            break;
        case 5:
            r = 1;
            g = 0;
            b = q;
            break;
    }
    const c =
        '#' +
        ('00' + (~~(r * 255)).toString(16)).slice(-2) +
        ('00' + (~~(g * 255)).toString(16)).slice(-2) +
        ('00' + (~~(b * 255)).toString(16)).slice(-2);
    return c;
}

/// A Handle is a thing that controls a player. For example, a text interface may be a handle.
/// A remote connection would also be a handle.
/// The object has a method `step`, which does all computation relevant to it,
/// and a method `render`, which does rendering.
function TextPlayerHandle(player, bindings = KEYBINDS) {
    const stats = Object.fromEntries(Object.entries(UNIT_STAT_BOUNDS).map(([k, v]) => [k, v[0]]));
    document.body.addEventListener('keydown', (e) => {
        const bind = bindings[e.key];
        if (bind !== undefined) {
            const b = bind[0];
            if (typeof b === 'string') {
                stats[b] += UNIT_STAT_BOUNDS[b][2];
                if (stats[b] > UNIT_STAT_BOUNDS[b][1]) {
                    stats[b] = UNIT_STAT_BOUNDS[b][0];
                }
            } else {
                b(stats, player);
            }
        }
    });
    return {
        step() {},
        render() {
            return $.div(
                `Current Price       : ${computePrice(stats).toString().substr(0, 3)}`,
                Object.entries(bindings).map((a) =>
                    $.div(
                        `${a[1][1]} - ${a[0]}`.padEnd(20) +
                            (typeof a[1][0] === 'string' ? ': ' + stats[a[1][0]].toString().substr(0, 3) : '')
                    )
                )
            );
        },
    };
}
function AIHandle(player) {
    function randomUnit() {
        const stats = {};
        for (const [k, v] of Object.entries(UNIT_STAT_BOUNDS)) {
            stats[k] = Math.floor((Math.random() * (v[1] - v[0] + 1) + v[0]) / v[2]) * v[2];
        }
        console.log(stats);
        return stats;
    }
    let next = randomUnit();
    return {
        step() {
            while (computePrice(next) <= player.resources) {
                player.unit(next);
                next = randomUnit();
            }
        },
        render() {
            return $.div(`[-- AI Handle --]`);
        },
    };
}
function EmptyHandle() {
    return {
        step() {},
        render() {
            return $.div();
        },
    };
}

/// An AI is a function which specifies what to do.
/// Its arguments list consists of the unit, a move function, and a firing function:
/// The move function takes in a single argument which is the position to move to.
/// The firing function takes in a single argument which is the unit to fire at.
//
//| 1  | function ai(unit, move, fire) {
//| 2  |     ...
//| 3  | }
function NothingAI() {}
function SimpleAI(unit, move, fire) {
    const pos = unit.position;
    const spawn = unit.controller.spawn;
    if (pos === spawn) {
        const options = [];
        if (spawn > 0) {
            options.push(-unit.stats.movement);
        }
        if (spawn + 1 < BOARD_SIZE) {
            options.push(unit.stats.movement);
        }
        move(options[Math.floor(Math.random() * options.length)]);
    } else {
        move(unit.stats.movement * Math.sign(pos - spawn));
    }
    const r = withinRange(unit);
    if (r.length > 0) {
        fire(r.reduce((a, b) => (a.stats.priority > b.stats.priority + Math.random() * 4 - 2 ? a : b)));
    }
}

function player(pid, resourceGain = RESOURCE_GAIN_PER_STEP, resources = STARTING_RESOURCES) {
    const res = {
        spawn: Math.floor((pid / (TOTAL_PLAYERS - 1)) * (BOARD_SIZE - 1)),
        id: pid,
        color: rainbow(TOTAL_PLAYERS, pid),
        resources,
        unit(_stats) {
            if (res.entity.destroyed) {
                console.log('Player Base destroyed; cannot create units.');
                return;
            }
            const stats = JSON.parse(JSON.stringify(_stats));
            for (const k of Object.keys(stats)) {
                if (UNIT_STAT_BOUNDS[k]) {
                    const bound = UNIT_STAT_BOUNDS[k];
                    stats[k] = minmax(stats[k], ...bound);
                }
            }
            const price = computePrice(stats);
            if (price <= res.resources) {
                res.resources -= price;
                stats.priority = price;
                unit(res, stats);
            } else {
                console.error(stats);
                console.error(`is too expensive for player ${pid}`);
            }
        },
        render() {
            if (res.entity.destroyed) {
                return $.div(`Player ${res.id}: ELIMINATED`);
            } else {
                return $.div(`Player ${res.id}'s Resources: ${res.resources.toString().substr(0, 3)}`);
            }
        },
    };
    const clone = JSON.parse(JSON.stringify(BASE_STATS));
    clone.ondestroy = () => {
        alivePlayers--;
    };
    clone.onstep = () => (res.resources += resourceGain);
    clone.ai = NothingAI;
    res.entity = unit(res, clone);
    return res;
}

function unit(player, stats, _location = player.spawn) {
    const location = bump(_location);
    const unit = {
        controller: player,
        ai: stats.ai ?? DEFAULT_AI,
        health: stats.health,
        position: location,
        partial: 0,
        destroyed: false,
        ondestroy: stats.ondestroy ?? (() => {}),
        onstep: stats.onstep ?? (() => {}),
        stats,
        damage(dmg) {
            if (unit.destroyed) throw new Error('Invalid Unit');
            unit.health -= dmg;
        },
        resolveDamage() {
            if (unit.health < 0) {
                units.delete(unit);
                board[unit.position] = null;
                unit.destroyed = true;
                unit.position = -1;
                unit.ai = () => {
                    throw new Error('Invalid Unit');
                };
                unit.ondestroy();
            }
        },
        step() {
            if (unit.destroyed) throw new Error('Invalid Unit');
            const phases = [];
            let fired = false;
            function fire(other) {
                if (fired) throw new Error('Cannot fire twice.');
                fired = true;
                phases.push(() => {
                    if (Math.abs(unit.position - other.position) > unit.range) {
                        throw new Error('Range out of bounds.');
                    }
                    other.damage(unit.stats.firepower);
                });
            }
            let moved = false;
            function move(ds) {
                if (moved) throw new Error('Cannot move twice.');
                moved = true;
                phases.push(() => {
                    if (Math.abs(ds) > unit.stats.movement) throw new Error('Cannot move more than movement stat.');
                    //console.log(ds);
                    const part = unit.partial + ds;
                    //console.log(part);
                    const delta = Math.sign(part) * Math.floor(Math.abs(part));
                    console.log(delta);
                    const pos = unit.position + delta;
                    board[unit.position] = null;
                    const actual = bump(minmax(pos, 0, BOARD_SIZE - 1), ds);
                    if (actual === undefined) {
                        board[unit.position] = unit;
                    } else {
                        unit.position = actual;
                        unit.partial = part - delta;
                        board[actual] = unit;
                    }
                });
            }
            unit.ai(unit, move, fire);
            phases.push(unit.onstep);
            phases.push(unit.resolveDamage);
            return phases;
        },
        render() {
            let text = '';
            for (const x of Object.values(UNIT_PROPERTY_NAMES)) {
                const d = getDeep(unit, x);
                text += `${
                    typeof d === 'number' ? d.toString().replace(/^0/, '').substr(0, UNIT_PAD).padEnd(UNIT_PAD) : d
                }\n`;
            }
            return $.div.unit(
                {
                    style: {
                        __color: player.color,
                    },
                },
                text
            );
        },
    };
    units.add(unit);
    board[location] = unit;
    return unit;
}

let paused = false;
let _run = 0;
document.body.addEventListener('keydown', (e) => {
    if (e.key === ' ') {
        paused = !paused;
        if (!paused) {
            clearTimeout(_run);
            run();
        }
    }
});
function run() {
    if (paused) return;
    handles.forEach((a) => a.step());
    const phaseSteps = [];
    const allSteps = Array.from(units).map((a) => a.step());
    do {
        phaseSteps.forEach((a) => a());
        phaseSteps.length = 0;
        for (const x of allSteps) {
            const s = x.shift();
            if (s !== undefined) {
                phaseSteps.push(s);
            }
        }
    } while (phaseSteps.length !== 0);
    _run = setTimeout(run, STEP);
}
const root = document.body;
(function render() {
    const boardRender = $.div$board(
        $.div.tile$unitStats(Object.keys(UNIT_PROPERTY_NAMES).join('\n')),
        ...board.map((a) => $.div.tile(a === null ? [] : a.render()))
    );
    const handlesRender = $.div.handles(
        ...handles.map((a, i) => $.div.handle($.div.handleInterior(players[i].render(), a.render())))
    );

    m.render(
        root,
        $.div.interior(
            boardRender,
            handlesRender,
            paused ? $.div$paused('PAUSED') : '',
            alivePlayers <= 1 ? $.div$gameOver('GAME OVER') : ''
        )
    );
    window.requestAnimationFrame(render);
})();
run();
