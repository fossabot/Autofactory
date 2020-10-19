/* global HTML, m */

const $ = HTML({
    h: m,
    textConvert: (a) => a,
});

const BOARD_SIZE = 10;
const DEFAULT_AI = SimpleAI;
const STARTING_RESOURCES = 10;
const RESOURCE_GAIN_PER_STEP = 3;
const TOTAL_PLAYERS = 2;
const BASE_STATS = {
    movement: 0,
    range: 0,
    firepower: 0,
    health: 40,
    priority: 999,
};
const UNIT_STAT_BOUNDS = {
    movement: [0, 3],
    range: [0, 5],
    firepower: [0, 5],
    health: [0, 10],
};

const board = Array(BOARD_SIZE)
    .fill(0)
    .map(() =>
        Array(TOTAL_PLAYERS)
            .fill(0)
            .map(() => new Set())
    );
const units = new Set();
const players = [player(0), player(1)];
const handles = [HTMLPlayerHandle(players[0]), AIHandle(players[1])];

/// Utils

function minmax(a, min, max) {
    return Math.max(min, Math.min(a, max));
}

function withinRange(unit) {
    const range = unit.stats.range;
    const pos = unit.position;
    const res = [];
    for (let i = pos - range; i <= pos + range; i++) {
        if (board[i] !== undefined) {
            board[i].forEach((a, i) => {
                if (i !== unit.controller.id) a.forEach((a) => res.push(a));
            });
        }
    }
    return res;
}

function computePrice(stats) {
    return stats.movement + stats.range + stats.firepower + stats.health;
}

/// A Handle is a function of a player that returns an object.
/// The object has a method, `render`, which renders the object.
function HTMLPlayerHandle(player) {
    return {
        render() {
            return $.div();
        },
    };
}
function AIHandle(player) {
    return {
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
        fire(r.reduce((a, b) => (a.stats.priority > b.stats.priority ? a : b)));
    }
}

function player(pid) {
    const res = {
        spawn: pid * (BOARD_SIZE - 1),
        id: pid,
        color: rainbow(TOTAL_PLAYERS, pid),
        resources: STARTING_RESOURCES,
        unit(stats) {
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
    };
    const clone = JSON.parse(JSON.stringify(BASE_STATS));
    clone.ondestroy = () => alert(`Player ${pid} loses the game.`);
    clone.onstep = () => (res.resources += RESOURCE_GAIN_PER_STEP);
    clone.ai = NothingAI;
    unit(res, BASE_STATS);
    return res;
}

function unit(player, stats, location = player.spawn) {
    const unit = {
        controller: player,
        ai: stats.ai ?? DEFAULT_AI,
        health: stats.health,
        position: location,
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
                board[unit.position][unit.controller.id].delete(unit);
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
            let fired = false;
            function fire(other) {
                if (fired) throw new Error('Cannot fire twice.');
                fired = true;
                if (Math.abs(unit.position - other.position) > unit.range) {
                    console.error('Range out of bounds.');
                } else {
                    other.damage(unit.stats.firepower);
                }
            }
            let moved = false;
            function move(ds) {
                if (moved) throw new Error('Cannot move twice.');
                moved = true;
                const pos = unit.position + ds;
                const actual = minmax(pos, 0, BOARD_SIZE - 1);
                board[unit.position][unit.controller.id].delete(unit);
                unit.position = actual;
                board[unit.position][unit.controller.id].add(unit);
            }
            unit.ai(unit, move, fire);
            unit.onstep();
        },
        render() {
            return $.span.unit(`Unit: ${unit.health}`);
        },
    };
    units.add(unit);
    board[location][player.id].add(unit);
    return unit;
}

function rainbow(numOfSteps, step) {
    // This function generates vibrant, "evenly spaced" colours (i.e. no clustering). This is ideal for creating easily distinguishable vibrant markers in Google Maps and other apps.
    // Adam Cole, 2011-Sept-14
    // HSV to RBG adapted from: http://mjijackson.com/2008/02/rgb-to-hsl-and-rgb-to-hsv-color-model-conversion-algorithms-in-javascript
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

const interior = document.getElementById('interior');
const handlesElem = document.getElementById('handles');
let paused = false;
let _run = 0;
document.body.onkeydown = (e) => {
    if (e.key === ' ') {
        paused = !paused;
        if (!paused) {
            clearTimeout(_run);
            run();
        }
    }
};
function run() {
    if (paused) return;
    units.forEach((a) => a.step());
    units.forEach((a) => a.resolveDamage());
    m.render(
        interior,
        $.table(
            $.tr(
                ...board.map((a) => {
                    return $.td(Array.from(a[0]).map((a) => a.render()));
                })
            ),
            $.tr(
                ...board.map((a) => {
                    return $.td(Array.from(a[1]).map((a) => a.render()));
                })
            )
        )
    );
    m.render(handlesElem, $.div(...handles.map((a) => a.render())));
    _run = setTimeout(run, 1000);
}
run();
