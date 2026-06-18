const { app, BrowserWindow, ipcMain, shell } = require('electron');
const path = require('path');
const fs = require('fs');
const os = require('os');
const sharp = require('sharp');

let win;

function createWindow() {
  win = new BrowserWindow({
    minWidth: 800,
    minHeight: 600,
    width: 800,
    height: 600,
    frame: false,
    resizable: true,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  });

  win.loadFile('www/index.html');

  win.on('closed', () => {
    win = null;
  });
}

app.on('ready', createWindow);

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    app.quit();
  }
});

app.on('activate', () => {
  if (win === null) {
    createWindow();
  }
});

// 窗口控制
ipcMain.on('close-main-window', () => {
  app.quit();
});

ipcMain.on('min-main-window', () => {
  if (win) win.minimize();
});

// 打开路径
ipcMain.handle('open-path', async (event, filePath) => {
  return shell.openPath(filePath);
});

// 打开 URL
ipcMain.handle('open-url', async (event, url) => {
  return shell.openExternal(url);
});

// 生成输出目录名
function getOutputDir() {
  const now = new Date();
  const timestamp = now.getFullYear() + '-' +
    String(now.getMonth() + 1).padStart(2, '0') + '-' +
    String(now.getDate()).padStart(2, '0') + '-' +
    String(now.getHours()).padStart(2, '0') + '_' +
    String(now.getMinutes()).padStart(2, '0') + '_' +
    String(now.getSeconds()).padStart(2, '0');
  return path.join(os.homedir(), 'retrocode_io', 'imagemin', timestamp);
}

// 获取文件扩展名
function getExt(filePath) {
  return path.extname(filePath).toLowerCase();
}

// 压缩图片
async function compressImage(filePath, outputDir, quality) {
  const ext = getExt(filePath);
  const filename = path.basename(filePath);
  const outputPath = path.join(outputDir, filename);
  const inputBuffer = await fs.promises.readFile(filePath);
  let pipeline = sharp(inputBuffer);

  switch (ext) {
    case '.jpg':
    case '.jpeg':
      pipeline = pipeline.jpeg({ quality: quality.jpgq });
      break;
    case '.png':
      pipeline = pipeline.png({ quality: quality.pngq });
      break;
    case '.gif':
      pipeline = pipeline.gif();
      break;
    case '.svg':
      // SVG 保持原样
      await fs.promises.writeFile(outputPath, inputBuffer);
      return { size: inputBuffer.length };
    default:
      throw new Error(`不支持的格式: ${ext}`);
  }

  const outputBuffer = await pipeline.toBuffer();
  await fs.promises.writeFile(outputPath, outputBuffer);
  return { size: outputBuffer.length };
}

// 处理压缩请求
ipcMain.handle('compress-files', async (event, files, quality) => {
  const outputDir = getOutputDir();
  await fs.promises.mkdir(outputDir, { recursive: true });

  const results = [];

  for (const file of files) {
    try {
      const result = await compressImage(file.path, outputDir, quality);
      results.push({
        name: file.name,
        size: file.size,
        comp_size: result.size,
        path: file.path
      });
    } catch (err) {
      console.error(`压缩失败: ${file.name}`, err);
      results.push({
        name: file.name,
        size: file.size,
        comp_size: file.size,
        path: file.path,
        error: err.message
      });
    }
  }

  return { results, outputDir };
});
