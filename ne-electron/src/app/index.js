const canvas = document.getElementById('canvas');
const offscreen = canvas.transferControlToOffscreen();
const worker = new Worker('renderer.js');

function computeResizePacket(override = false) {
    const pixelRatio = devicePixelRatio;
    const width = (canvas.clientWidth * pixelRatio) | 0;
    const height = (canvas.clientHeight * pixelRatio) | 0;
    const needResize = canvas.width !== width || canvas.height !== height;
    if (needResize || override) {
        return {
            width,
            height,
            clientWidth: canvas.clientWidth,
            clientHeight: canvas.clientHeight,
            pixelWidth: canvas.width,
            pixelHeight: canvas.height,
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
