const { load } = require('./utils.js');
const THREE = require('three');
console.log(THREE);
console.log('Test');
const gl = document.getElementById('canvas').getContext('webgl2', {
    powerPreference: 'high-performance',
});
