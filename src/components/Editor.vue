<script setup>
import { defineProps, ref, onMounted } from 'vue';
import Vditor from 'vditor';
import 'vditor/dist/index.css';

const editor = ref("");

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
    }
  });
})

</script>

<template>
  <div id="editor"/>
</template>

<style scoped>
#editor {
  height: 100%;
}
</style>
