import { createModuleWorker } from './utils.js';

const canvas = document.getElementById('canvas');
const offscreen = canvas.transferControlToOffscreen();
const worker = createModuleWorker('./renderer.js');

function computeResizePacket(override = false) {
    const width = canvas.clientWidth;
    const height = canvas.clientHeight;
    const needResize = canvas.width !== width || canvas.height !== height;
    if (needResize || override) {
        return {
            width,
            height,
        };
    }
    return null;
}

worker.postMessage({ canvas: offscreen, canvasData: computeResizePacket(true) }, [offscreen]);

(function poll() {
    const packet = computeResizePacket();
    if (packet != null) worker.postMessage(['canvas-resize', packet]);
    requestAnimationFrame(poll);
})();
