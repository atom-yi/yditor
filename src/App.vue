<script setup>
import Editor from './components/Editor.vue';
import Sidebar from './components/Sidebar.vue';
import { onMounted, ref } from 'vue';

// const
const leftMinWidth = 250;

// data
const showSider = ref(true);

// mounted
onMounted(() => {
  dragControl();
});

// methods
function collapse() {
  showSider.value = !showSider.value;
  console.log(showSider.value);
  let mainContent = document.getElementById('main-content');
  let mainControl = document.getElementById('main-control');
  if (showSider.value) {
    dragControl();
    mainContent.style.width = '75%';  
  } else {
    mainControl.onmousedown = null;
    mainContent.style.width = '100%';
  }
}

// internal methods
function dragControl() {
  let mainControl = document.getElementById('main-control');
  let main = document.getElementById('main');
  let mainSider = document.getElementById('main-sider');
  let mainContent = document.getElementById('main-content');

  mainControl.onmousedown = function(e) {
    // 拖拽开始位置
    let startX = e.clientX;
    mainControl.left = mainControl.offsetLeft;

    // 鼠标拖拽
    document.onmousemove = function(ee) {
      let endX = ee.clientX;
      let leftWidth = mainControl.left + (endX - startX);
      let maxWidth = main.clientWidth - mainControl.offsetWidth;
      if (leftWidth < leftMinWidth) leftWidth = leftMinWidth;
      if (leftWidth > maxWidth - leftMinWidth) leftWidth = maxWidth - leftMinWidth;
      mainControl.style.left = leftWidth;
      mainSider.style.width = leftWidth + 'px';
      mainContent.style.width = (main.clientWidth - leftWidth - 10) + 'px';
    };

    // 鼠标松开
    document.onmouseup = function(e) {
      document.onmousemove = null;
      document.onmouseup = null;
    }
  }
}

</script>

<template>
  <div id="main">
    <div
      id="main-sider"
      :width="260"
      theme="light" 
      v-show="showSider">
      <Sidebar/>
    </div>
  <div id="main-control" @dblclick="collapse">
    <span class="tool-tip-text">左右拖动 双击收起</span>
  </div>
  <div id="main-content"><Editor/></div>
  </div>
</template>

<style scoped>
#main {
  height: 100%;
  display: flex;
}
#main-sider {
  width: 25%;
  border-right: 1px solid #d1d5da;
}
#main-content {
  width: 75%;
}
#main-control {
  cursor: col-resize;
  position: relative;
  top: 45%;
  background: pink;
  border-radius: 5px;
  width: 5px;
  height: 100px;
  background-size: cover;
  background-position: center;
  font-size: 32px;
  color: white;
}
#main-control:hover {
  color: #444444;
}
#main-control .tool-tip-text {
  visibility: hidden;
  text-align: center;
  border-radius: 6px;
  background-color: black;
  color: #fff;
  width: 60px;
  position: absolute;
  font-size: 12px;
  z-index: 1;
}
#main-control:hover .tool-tip-text {
  visibility: visible;
}
</style>