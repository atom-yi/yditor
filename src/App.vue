<script setup>
import Editor from './components/Editor.vue';
import Sidebar from './components/Sidebar.vue';
</script>

<template>
  <a-layout id="main">
    <a-layout-sider v-show="true"><Sidebar/></a-layout-sider>
    <a-layout-content><Editor/></a-layout-content>
  </a-layout>
</template>

<style scoped>
#main {
  height: 100%;
}
</style>