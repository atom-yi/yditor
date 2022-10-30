<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api';
import { save } from '@tauri-apps/api/dialog';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
import emitter from '../util/mitt'

// internal data
const saveIcon = '<svg t="1667120694317" class="icon" viewBox="0 0 1024 1024" version="1.1" xmlns="http://www.w3.org/2000/svg" p-id="5071" width="64" height="64"><path d="M906.666667 298.666667L725.333333 117.333333c-14.933333-14.933333-32-21.333333-53.333333-21.333333H170.666667C130.133333 96 96 130.133333 96 170.666667v682.666666c0 40.533333 34.133333 74.666667 74.666667 74.666667h682.666666c40.533333 0 74.666667-34.133333 74.666667-74.666667V349.866667c0-19.2-8.533333-38.4-21.333333-51.2zM652.8 864H371.2V648.533333h281.6v215.466667z m211.2-10.666667c0 6.4-4.266667 10.666667-10.666667 10.666667h-140.8V618.666667c0-17.066667-12.8-29.866667-29.866666-29.866667H341.333333c-17.066667 0-29.866667 12.8-29.866666 29.866667v245.333333H170.666667c-6.4 0-10.666667-4.266667-10.666667-10.666667V170.666667c0-6.4 4.266667-10.666667 10.666667-10.666667h140.8V320c0 17.066667 12.8 29.866667 29.866666 29.866667h277.333334c17.066667 0 29.866667-12.8 29.866666-29.866667s-12.8-29.866667-29.866666-29.866667H371.2V160h302.933333c2.133333 0 6.4 2.133333 8.533334 2.133333l179.2 179.2c2.133333 2.133333 2.133333 4.266667 2.133333 8.533334V853.333333z" p-id="5072"></path></svg>';
let filepath = '';
let toolbar = [
  {
    name: 'bold',
  },
  {
    name: 'italic',
  },
  {
    name: 'strike',
  },
  {
    name: '|',
  },
  {
    name: 'table',
  },
  {
    name: 'line',
  },
  {
    name: 'quote',
  },
  {
    name: '|',
  },
  {
    name: 'insert-after',
  },
  {
    name: 'insert-before',
  },
  {
    name: '|',
  },
  {
    name: 'outline',
  },
  {
    name: 'edit-mode',
  },
  {
    name: 'content-theme',
  },
  {
    name: 'code-theme',
  },
  {
    name: 'help',
  },
  {
    name: 'save',
    icon: saveIcon,
    tip: '保存',
    click: () => {
      saveFile(editor.value.getValue());
    }
  },
];
toolbar.forEach(value => value.tipPosition = 's');

// data
const editor = ref("");

// mounted
onMounted(() => {
  editor.value = new Vditor('editor', {
    height: '100%',
    placeholder: '请在此输入内容',
    mode: 'wysiwyg',
    toolbar: toolbar,
    toolbarConfig: {
      hide: false,
      pin: true,
    },
    cache: {
      enabledd: false,
    },
    outline: {
      position: 'right',
    }
  });
});

// unmounted
onUnmounted(() => {
  saveFile(editor.value.getValue());
})

// event listening
emitter.on('loadFileContentToEditor', event => {
  filepath = event;
  invoke('read_file', { filepath: event })
  .then(resp => {
    editor.value.setValue(resp);
  })
  .catch(err => {
    message.error(err);
  });
});

// internal method
function saveFile(content) {
  if (filepath != '') {
    invoke('save_to_file', {
      filepath: filepath,
      content: content,
    })
    .then(resp => message.success('保存成功: ' + filepath))
    .catch(err => message.error('保存失败: ' + err));
  } else {
    save({
      multiple: false,
      title: '保存',
      filters: [
        {
          name: 'Markdown',
          extensions: ['md'],
        }
      ]
    })
    .then(resp => {
      if (resp) {
        let newFilepath = resp;
        if (!resp.endsWith('.md')) {
          newFilepath += '.md';
        }
        invoke('save_to_file', {
          filepath: newFilepath,
          content: content,
        })
        .then(resp => {
          filepath = newFilepath;
          message.success('保存成功: ' + newFilepath);
        })
        .catch(err => message.error('保存失败: ' + err));
      }
    })
    .catch(err => message.error('保存失败: ' + err));
  }
}

</script>

<template>
  <div id="editor"/>
</template>

<style scoped>
#editor {
  height: 100%;
}
</style>
