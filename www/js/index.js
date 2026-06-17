'use strict';

const version = '3.0';

function getFileSize(bytes) {
  const exp = Math.floor(Math.log(bytes) / Math.log(1024)) | 0;
  const result = (bytes / Math.pow(1024, exp)).toFixed(2);
  return result + ' ' + (exp === 0 ? 'bytes' : 'KMGTPEZY'[exp - 1] + 'B');
}

function basename(path) {
  const name = path.split(/[\\/]/).pop();
  return name.substring(0, name.lastIndexOf('.'));
}

function toast(message, timeout = 3000) {
  mdui.snackbar(message, { timeout, position: 'bottom' });
}

function drop_handler(ev) {
  ev.preventDefault();
  const dt = ev.dataTransfer;
  const files = [];
  if (dt.items) {
    for (let i = 0; i < dt.items.length; i++) {
      if (dt.items[i].kind === 'file') {
        files.push(dt.items[i].getAsFile());
      }
    }
  } else {
    files = dt.files;
  }
  compFile(files);
}

function dragover_handler(ev) {
  ev.preventDefault();
}

function dragend_handler(ev) {
  const dt = ev.dataTransfer;
  if (dt.items) {
    for (let i = 0; i < dt.items.length; i++) {
      dt.items.remove(i);
    }
  } else {
    ev.dataTransfer.clearData();
  }
}

function validateFile(files) {
  if (!files || files.length === 0) return false;

  for (let i = 0; i < files.length; i++) {
    const base1 = basename(files[i].name);
    for (let j = i + 1; j < files.length; j++) {
      const base2 = basename(files[j].name);
      if (base1 === base2) {
        toast('文件基本名不能相同');
        return false;
      }
    }
  }

  return true;
}

function getFilesInfo(files) {
  const result = [];
  for (let i = 0; i < files.length; i++) {
    result.push({
      path: files[i].path,
      name: files[i].name,
      size: files[i].size,
      type: files[i].type
    });
  }
  return result;
}

async function compFile(files) {
  if (!validateFile(files)) return;

  $('#result').addClass('hide');
  $('#progress').removeClass('hide');

  const fileInfo = getFilesInfo(files);

  const jpgQ = Math.max(1, Math.min(100, parseInt(localStorage.getItem('jpg-quality')) || 80));
  const pngQ = Math.max(21, Math.min(100, parseInt(localStorage.getItem('pngQ-quality')) || 80));
  const pngQ_min = pngQ - 20;
  const webpQ = Math.max(1, Math.min(100, parseInt(localStorage.getItem('webpQ-quality')) || 80));

  const quality = {
    jpgq: jpgQ,
    pngq: pngQ,
    pngq_min: pngQ_min,
    webpq: webpQ
  };

  try {
    const { results, outputDir } = await window.electronAPI.compressFiles(fileInfo, quality);

    $('#filelist').empty();
    for (const item of results) {
      const percent = Math.floor((item.comp_size - item.size) * 100 / item.size) + '%';
      const row = `<tr><td>${item.name}</td><td>${getFileSize(item.size)}</td><td>${getFileSize(item.comp_size)}</td><td>${percent}</td></tr>`;
      $('#filelist').append(row);
    }

    if (results.length > 0) {
      $('#explore').data('folder', outputDir);
    } else {
      $('#explore').data('folder', '');
    }

    $('#summary').text(`共成功压缩 ${results.length} 个文件`);
    $('#progress').addClass('hide');
    $('#result').removeClass('hide');
  } catch (error) {
    console.error('压缩失败:', error);
    toast('出错了，可能是系统资源不足');
    $('#progress').addClass('hide');
  }
}

function onQualityChange() {
  const jpgQ = $('input[name=jpg]').val();
  const pngQ = $('input[name=png]').val();
  const webpQ = $('input[name=webp]').val();

  $('#jpg-val').text(jpgQ);
  $('#png-val').text(pngQ);
  $('#webp-val').text(webpQ);

  localStorage.setItem('jpg-quality', jpgQ);
  localStorage.setItem('pngQ-quality', pngQ);
  localStorage.setItem('webpQ-quality', webpQ);
}

function loadSetting() {
  const jpgQ = Math.max(1, Math.min(100, parseInt(localStorage.getItem('jpg-quality')) || 80));
  const pngQ = Math.max(1, Math.min(100, parseInt(localStorage.getItem('pngQ-quality')) || 80));
  const webpQ = Math.max(1, Math.min(100, parseInt(localStorage.getItem('webpQ-quality')) || 80));

  $('input[name=jpg]').val(jpgQ);
  $('input[name=png]').val(pngQ);
  $('input[name=webp]').val(webpQ);

  $('#jpg-val').text(jpgQ);
  $('#png-val').text(pngQ);
  $('#webp-val').text(webpQ);
  mdui.updateSliders();
}

$(document).ready(function () {
  $('#btn-close').click(function () {
    window.electronAPI.closeWindow();
  });

  $('#btn-min').click(function () {
    window.electronAPI.minimizeWindow();
  });

  $('#drop-upload-files').click(function () {
    $('#input-upload-files').val(null);
    $('#input-upload-files').click();
    return false;
  });

  $('#input-upload-files').change(function () {
    const files = $('#input-upload-files')[0].files;
    compFile(files);
    return false;
  });

  $('#explore').click(async function () {
    const folder = $(this).data('folder');
    if (folder !== '') {
      await window.electronAPI.openPath(folder);
    }
    return false;
  });

  $('#setting').click(function () {
    loadSetting();
    const tab = new mdui.Tab('#example4-tab');
    document.getElementById('setting-modal1').addEventListener('open.mdui.dialog', function () {
      tab.handleUpdate();
    });
    const inst = new mdui.Dialog('#setting-modal1', {});
    inst.open();
    return false;
  });

  $('input[name=jpg]').on('input change', onQualityChange);
  $('input[name=png]').on('input change', onQualityChange);
  $('input[name=webp]').on('input change', onQualityChange);

  $(document).on('click', 'a[href^="http"]', function (event) {
    event.preventDefault();
    window.electronAPI.openPath(this.href);
  });
});
