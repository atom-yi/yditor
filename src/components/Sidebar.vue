<script setup>
import { reactive,  onMounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';

// data
const treeData = reactive({
    baseDir: '',
    list: [],
    expendedFolderKeys: [],
    selectedFilesKeys: [],
});

// mounted
onMounted(() => {

});

// methods
function listFiles(dir) {
    return invoke("list_files_in_directory", {
        dir: dir.key,
    })
    .then((resp) => {
        buildFileEntries(resp);
    });
}
function openFolder() {
    open({
        directory: true,
        multiple: false,
    }).then(selected => {
        if (selected) {
            openFileTree(selected);
        }
    }).catch(err => {
        console.log(err);
    });
    return false;
}

// internal methods
function buildFileEntries(fileInfos) {
    // 找不到文件
    if (fileInfos.length < 0) {
        return;
    }

    let parentPath = fileInfos[0].parent_path;
    // 文件路径打开错了
    if (!parentPath.startsWith(treeData.baseDir)) { 
        return;
    }

    // 加载根目录文件
    if (parentPath === treeData.baseDir) {
        treeData.list[0].children = fileInfosToTreeData(fileInfos);
        return;
    }

    // 遍历路径追加节点
    let parent = findParent(treeData.list[0], parentPath);
    if (parent != null) {
        parent.children = fileInfosToTreeData(fileInfos);
    }
}
function fileInfosToTreeData(fileInfos) {
    let entries = [];
    for (let i = 0; i < fileInfos.length; i++) {
        entries.push(fileInfoToEntryInfo(fileInfos[i]));
    }
    return entries;
}
function findParent(node, parentPath) {
    if (parentPath === node.key) {
        return node;
    }
    
    let children = node.children;
    if (children.length == 0 || node.isLeaf) {
        return null;
    }

    for (let i = 0; i < children.length; i++) {
        let child = children[i];
        if (child.isLeaf || !parentPath.startsWith(child.key)) {
            console.log("不满足")
            continue;
        }
        return findParent(child, parentPath);
    }
    return null;
}
function fileInfoToEntryInfo(fileInfo) {
    return {
        title: fileInfo.name,
        key: fileInfo.path,
        isLeaf: !fileInfo.is_directory,
        children: [],
    };
}
function openFileTree(baseDir) {
    treeData.baseDir = baseDir;
    treeData.list = [
        {
            title: '目录',
            key: baseDir,
            isLeaf: false,
            children: []
        }
    ];
    listFiles(treeData.list[0]);
}

</script>

<template>
    <div id="sidebar">
        <a-button type="primary" @click="openFolder">打开文件夹</a-button>
        <a-directory-tree
            :expendedKeys="treeData.expendedFolderKeys"
            :selectedKeys="treeData.selectedFilesKeys"
            multiple
            blockNode
            :tree-data="treeData.list"
            :load-data="listFiles"
        ></a-directory-tree>
    </div>
</template>

<style scoped>
#sidebar {
    height: 100vh;
    width: 100%;
    overflow: auto;
    overflow-y: auto;
}
</style>