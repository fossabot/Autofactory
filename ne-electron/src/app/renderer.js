onmessage = (startupMessage) => {
    const { canvas, canvasData } = startupMessage.data;
    Object.assign(canvas, canvasData);
    const THREE = require('three');
    const context = canvas.getContext('webgl2', {
        powerPreference: 'high-performance',
    });
    const renderer = new THREE.WebGLRenderer({ canvas, context });

    let camera;

    const scene = new THREE.Scene();
    scene.background = new THREE.Color(0x050505);

    {
        const fov = 75;
        const aspect = canvas.width / canvas.height;
        const near = 0.1;
        const far = 5;
        camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
        camera.position.z = 2;
        camera.zoom = fov / 75;
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

    let time = Date.now();

    (function render() {
        const now = Date.now();
        const dt = now - time;
        time = now;

        const speed = 0.001;

        cube.rotation.x += dt * speed;
        cube.rotation.y += dt * speed;

        renderer.render(scene, camera);
        console.log('Rendering');
        requestAnimationFrame(render);
    })();

    onmessage = (message) => {
        const msg = message.data[1];
        switch (message.data[0]) {
            case 'canvas-resize':
                Object.assign(canvas, msg);
                renderer.setSize(msg.width, msg.height, false);
                camera.aspect = canvas.clientWidth / canvas.clientHeight;
                camera.updateProjectionMatrix();
                break;
        }
    };
};
