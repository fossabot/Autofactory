import * as THREE from '../../node_modules/three/build/three.module.js';
import { EffectComposer } from '../../node_modules/three/examples/jsm/postprocessing/EffectComposer.js';
import { RenderPass } from '../../node_modules/three/examples/jsm/postprocessing/RenderPass.js';
import { ShaderPass } from '../../node_modules/three/examples/jsm/postprocessing/ShaderPass.js';
import { CopyShader } from '../../node_modules/three/examples/jsm/shaders/CopyShader.js';

const msaa = false;

function loadImage(url) {
    return new Promise((resolve) => {
        postMessage(['load-image', url]);
        onmessage = (message) => {
            const msg = message.data;
            if (msg[0] != 'load-image') throw new Error('Invalid Message');
            msg[1].data = new Uint8ClampedArray(msg[1].data);
            resolve(msg[1]);
        };
    });
}

onmessage = async (startupMessage) => {
    onmessage = () => {};
    const { canvas, canvasData } = startupMessage.data;
    Object.assign(canvas, canvasData);
    const context = canvas.getContext('webgl2', {
        powerPreference: 'high-performance',
        antialias: false,
    });
    const renderer = new THREE.WebGLRenderer({ canvas, context });

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x050505);

    let camera;
    {
        const fov = 75;
        const aspect = canvas.width / canvas.height;
        const near = 0.1;
        const far = 5;
        camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
        camera.position.z = 2;
        camera.zoom = fov / 75;
    }

    const target = new (msaa ? THREE.WebGLMultisampleRenderTarget : THREE.WebGLRenderTarget)(
        canvas.width,
        canvas.height
    );

    const composer = new EffectComposer(renderer, target);
    composer.addPass(new RenderPass(scene, camera));
    composer.addPass(new ShaderPass(CopyShader));

    let cube;
    {
        const image = await loadImage('textures/cubeTexture.png');
        const cubeTexture = new THREE.DataTexture(image.data, image.width, image.height, THREE.RGBAFormat);
        const boxWidth = 1;
        const boxHeight = 1;
        const boxDepth = 1;
        const geometry = new THREE.BoxBufferGeometry(boxWidth, boxHeight, boxDepth);
        const material = new THREE.MeshPhongMaterial({ color: 0x304030, map: cubeTexture });
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
        scene.add(new THREE.AmbientLight(0xfffff));
    }

    let time = performance.now();

    (function render() {
        const now = performance.now();
        const dt = now - time;
        time = now;

        const speed = 0.001;

        cube.rotation.x += dt * speed;
        cube.rotation.y += dt * speed;

        composer.render();
        requestAnimationFrame(render);
    })();

    postMessage(['ready']);
    onmessage = (message) => {
        const msg = message.data[1];
        switch (message.data[0]) {
            case 'canvas-resize':
                Object.assign(canvas, msg);
                renderer.setSize(msg.width, msg.height, false);
                target.setSize(msg.width, msg.height);
                camera.aspect = canvas.width / canvas.height;
                camera.updateProjectionMatrix();
                break;
        }
    };
};
