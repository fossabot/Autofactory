'use strict';
const isFullScreen = false;
const isBorderless = true;
const shouldShortcut = true; // Set to false to disable devtools (and prevent closing)
const { app, BrowserWindow /*, screen*/ } = require('electron');
let win;

function createWindow() {
    const { width, height } = { width: 1200, height: 800 }; //screen.getPrimaryDisplay().workAreaSize;
    win = new BrowserWindow({
        width,
        height,
        frame: !isBorderless,
        transparent: false,
        webPreferences: {
            nodeIntegration: true,
            nodeIntegrationInWorker: true,
        },
    });
    if (shouldShortcut) {
        win.setMenuBarVisibility(false);
    } else {
        win.setMenu(null);
    }
    if (isFullScreen) {
        win.maximize();
    }
    win.loadFile('./app/index.html');
    win.show();
    win.on('closed', () => {
        win = null;
    });
    win.once('ready-to-show', win.show);
}
app.on('ready', () => {
    setTimeout(createWindow, 1000);
});
app.on('window-all-closed', () => {
    app.quit();
    process.exit(0);
});
app.on('activate', () => {
    if (win === null) {
        createWindow();
    }
});
