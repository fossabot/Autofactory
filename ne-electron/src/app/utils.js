const fs = require('fs');

module.exports = {
    load(path) {
        return fs.readFileSync(__dirname + '/' + path);
    },
};
