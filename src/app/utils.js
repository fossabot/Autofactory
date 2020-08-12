const fs = require('fs');

export function load(path) {
    return fs.readFileSync(__dirname + '/' + path);
}

export function createModuleWorker(filename) {
    const worker = new Worker('ModuleWorker.js');
    worker.postMessage(filename);
    return worker;
}
