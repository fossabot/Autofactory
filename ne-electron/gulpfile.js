const { src, dest, parallel } = require('gulp');
const sourcemaps = require('gulp-sourcemaps');
const gstylus = require('gulp-stylus');
const gpug = require('gulp-pug');

const copy = () => src('src/**/*').pipe(dest('out/'));
const stylus = () =>
    src('src/**/*.styl').pipe(sourcemaps.init()).pipe(gstylus()).pipe(sourcemaps.write()).pipe(dest('out/'));
const pug = () => src('src/**/*.pug').pipe(gpug()).pipe(dest('out/'));
exports.default = parallel(copy, stylus, pug);
exports.copy = copy;
exports.stylus = stylus;
exports.pug = pug;
