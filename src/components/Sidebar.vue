<script setup>
import { reactive,  onMounted } from 'vue';
import { invoke } from '@tauri-apps/api';
import { open } from '@tauri-apps/api/dialog';
import emitter from '../util/mitt';
import { Tooltip, Dropdown, Menu, Tree } from 'ant-design-vue';
import { FolderOpenOutlined } from '@ant-design/icons-vue';

// data
const treeData = reactive({
    baseDir: '',
    list: [],
    expendedKeys: [],
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
function clickTreeNode(selectedKeys, event) {
    if (selectedKeys.length > 1) {
        // 不支持同时打开多个文件
        return;
    } else if (!event.node.isLeaf) {
        // 文件夹不能加载内容到编辑器
        return;
    }
    emitter.emit('loadFileContentToEditor', event.node.key);
}
function onContextMenuClick(treeKey, menuKey) {
    if (menuKey == 'createNewFolder') {
        createItem(treeKey, true);
    } else if (menuKey == 'createNewFile') {
        createItem(treeKey, false);
    } else if (menuKey == 'deleteItem') {
        deleteItem(treeKey);
    } else if (menuKey == 'refreshFolder') {
        refreshFolder(treeKey);
    }
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
        treeData.expendedKeys.push(treeData.baseDir);
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
function createItem(treeKey, isDirectory) {
    // todo
}
function deleteItem(filepath) {
    // todo
}
function refreshFolder(filepath) {
    // todo
}
</script>

<template>
    <div id="sidebar">
        <a-tooltip title="打开文件夹">
            <a-button type="primary" @click="openFolder">
                <folder-open-outlined />
            </a-button>    
        </a-tooltip>
        <a-tree
            blockNode
            v-model:expandedKeys="treeData.expendedKeys"
            :showLine="true"
            :autoExpandParent="true"
            :tree-data="treeData.list"
            :load-data="listFiles"
            @select="clickTreeNode"
        >
            <template #title="{ key: treeKey, title, isLeaf }">
                <a-dropdown :trigger="['contextmenu']">
                    <span>{{ title }}</span>
                    <template #overlay>
                        <a-menu @click="({ key: menuKey }) => onContextMenuClick(treeKey, menuKey)">
                            <a-menu-item key="createNewFolder" v-if="!isLeaf">
                                创建文件夹
                            </a-menu-item>
                            <a-menu-item key="createNewFile" v-if="!isLeaf">
                                创建文件
                            </a-menu-item>
                            <a-menu-item key="deleteItem">
                                删除{{ isLeaf ? '文件' : '文件夹' }}
                            </a-menu-item>
                            <a-menu-item key="refreshFolder">
                                刷新目录
                            </a-menu-item>
                        </a-menu>
                    </template>
                </a-dropdown>
            </template>
        </a-tree>
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