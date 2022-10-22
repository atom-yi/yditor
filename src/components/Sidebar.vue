<script setup>
import { message } from 'ant-design-vue';
import { invoke } from "@tauri-apps/api";

function greet() {
    invoke("greet", {
        name: "Atom",
    })
    .then((resp) => {
        message.info(resp);
    })
}
</script>

<template>
    <div id="sidebar">
        <a-button type="primary" @click="greet">greet</a-button>
    </div>
</template>

<style scoped>
#sidebar {
    height: 100%;
}
</style>