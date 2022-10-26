<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { message } from 'ant-design-vue';
import { invoke } from '@tauri-apps/api';
import Vditor from 'vditor';
import 'vditor/dist/index.css';
import emitter from '../util/mitt'

// data
let filepath = '';
const editor = ref("");

// mounted
onMounted(() => {
  editor.value = new Vditor('editor', {
    height: '100%',
    toolbar: ['bold', 'italic', 'strike', '|', 'table', 'line', 'quote', 'check', '|', 'insert-after', 'insert-before', '|', 'outline', 'edit-mode', 'content-theme', 'code-theme', 'help'],
    cache: {
      enabledd: false,
    },
    outline: {
      position: 'right',
    },
    after: () => {
      editor.value.setValue("Hello!");
    },
    blur: (content) => {
      saveFile(content);
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
    .then(resp => console.log(resp))
    .catch(err => console.log(err));
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
