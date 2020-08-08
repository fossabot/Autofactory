const THREE = require('three');
console.log(THREE);
console.log('Test');
const canvas = document.getElementById('canvas');
const context = canvas.getContext('webgl2', {
    powerPreference: 'high-performance',
});
const renderer = new THREE.WebGLRenderer({ canvas, context });

let camera;

const scene = new THREE.Scene();
scene.background = new THREE.Color(0x050505);

{
    const fov = 75;
    const aspect = canvas.clientWidth / canvas.clientHeight;
    const near = 0.1;
    const far = 5;
    camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
    camera.position.z = 2;
    camera.zoom = fov / 75;
}

{
    // const whr = canvas.clientHeight / canvas.clientWidth;
    // camera = new THREE.OrthographicCamera(-3, 3, 3 * whr, -3 * whr);
    // camera.position.z = 2;
}

let cube;
{
    const boxWidth = 1;
    const boxHeight = 1;
    const boxDepth = 1;
    const geometry = new THREE.BoxGeometry(boxWidth, boxHeight, boxDepth);
    const material = new THREE.MeshPhongMaterial({ color: 0x304030 });
    cube = new THREE.Mesh(geometry, material);
    scene.add(cube);
}
// Lights
{
    const color = 0xffffff;
    const intensity = 0.5;
    const light = new THREE.DirectionalLight(color, intensity);
    light.position.set(-1, 2, 4);
    scene.add(light);
    scene.add(new THREE.AmbientLight(0xffffff));
}

function resizeRendererToDisplaySize(renderer) {
    const canvas = renderer.domElement;
    const pixelRatio = window.devicePixelRatio;
    const width = (canvas.clientWidth * pixelRatio) | 0;
    const height = (canvas.clientHeight * pixelRatio) | 0;
    const needResize = canvas.width !== width || canvas.height !== height;
    if (needResize) {
        renderer.setSize(width, height, false);
    }
    return needResize;
}

let time = Date.now();

(function render() {
    const now = Date.now();
    const dt = now - time;
    time = now;

    if (resizeRendererToDisplaySize(renderer)) {
        const canvas = renderer.domElement;
        camera.aspect = canvas.clientWidth / canvas.clientHeight;
        // const whr = canvas.clientHeight / canvas.clientWidth;

        // camera.top = 3 * whr;
        // camera.bottom = -3 * whr;
        camera.updateProjectionMatrix();
    }

    const speed = 0.001;

    cube.rotation.x += dt * speed;
    cube.rotation.y += dt * speed;

    renderer.render(scene, camera);

    requestAnimationFrame(render);
})();