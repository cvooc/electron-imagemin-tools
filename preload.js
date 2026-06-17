const { contextBridge, ipcRenderer } = require('electron');

contextBridge.exposeInMainWorld('electronAPI', {
  compressFiles: (files, quality) => ipcRenderer.invoke('compress-files', files, quality),
  closeWindow: () => ipcRenderer.send('close-main-window'),
  minimizeWindow: () => ipcRenderer.send('min-main-window'),
  openPath: (path) => ipcRenderer.invoke('open-path', path)
});
