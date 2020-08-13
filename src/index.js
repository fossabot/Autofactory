import { createModuleWorker } from './utils.js';

const stats = new require('stats.js')();
stats.showPanel(0);
document.body.appendChild(stats.dom);
const ctx = document.getElementById('imageLoadCanvas').getContext('2d');
{
    const canvas = document.getElementById('imageLoadCanvas');
    canvas.width = 1000;
    canvas.height = 1000;
}

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
worker.onmessage = (message) => {
    const msg = message.data;
    switch (msg[0]) {
        case 'load-image':
            {
                const url = msg[1];
                const image = new Image();
                image.src = url;
                console.log('Loading image: ' + url);
                image.onload = () => {
                    ctx.drawImage(image, 0, 0);
                    const { width, height } = image;
                    const data = ctx.getImageData(0, 0, width, height).data;
                    worker.postMessage(['load-image', { url, data: data.buffer, width, height }], [data.buffer]);
                    console.log('Finished loading image: ' + url);
                };
            }
            break;
        case 'ready':
            (function poll() {
                const packet = computeResizePacket();
                if (packet != null) worker.postMessage(['canvas-resize', packet]);
                requestAnimationFrame(poll);
            })();
            break;
        case 'tick':
            stats.update();
    }
};
