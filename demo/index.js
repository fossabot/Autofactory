/* global HTML */

const $ = HTML;

const BOARD_SIZE = 10;
const DEFAULT_AI = SimpleAI;
const TOTAL_PLAYERS = 2;
const BASE_STATS = {
    movement: 0,
    range: 0,
    firepower: 0,
    health: 10,
    priority: 999,
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

function player(pid) {
    const res = {
        spawn: pid * (BOARD_SIZE - 1),
        id: pid,
        color: rainbow(TOTAL_PLAYERS, pid),
    };
    unit(res, BASE_STATS, res.spawn, NothingAI, () => {
        alert(`Player ${pid} loses the game.`);
    });
    return res;
}

function unit(player, stats, location = player.spawn, ai = DEFAULT_AI, ondestroy = () => {}) {
    const unit = {
        controller: player,
        ai,
        health: stats.health,
        position: location,
        destroyed: false,
        ondestroy,
        stats,
        damage(dmg) {
            if (unit.destroyed) throw new Error('Invalid Unit');
            unit.health -= dmg;
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
            function fire(other) {
                if (Math.abs(unit.position - other.position) > unit.range) {
                    console.error('Range out of bounds.');
                } else {
                    other.damage(unit.stats.firepower);
                }
            }
            function move(ds) {
                const pos = unit.position + ds;
                const actual = Math.min(BOARD_SIZE - 1, Math.max(0, pos));
                board[unit.position][unit.controller.id].delete(unit);
                unit.position = actual;
                board[unit.position][unit.controller.id].add(unit);
            }
            unit.ai(unit, move, fire);
        },
        display() {
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

function run() {
    for (const unit of units) {
        unit.step();
    }
    interior.innerHTML = '';
    interior.appendChild(
        $.table(
            $.tr(
                board.map((a) => {
                    return $.td(
                        Array.from(a[0])
                            .map((a) => a.display())
                    );
                })
            ),
            $.tr(
                board.map((a) => {
                    return $.td(
                        Array.from(a[1])
                            .map((a) => a.display())
                    );
                })
            )
        )
    );
}

setInterval(run, 1000);
