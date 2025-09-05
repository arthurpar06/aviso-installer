<template>
    <div class="flex items-center justify-between">
        <div class="text-sm text-gray-600">{{ filePath ? filePath : 'No LFXX.sct file selected' }}</div>
        <Button @click="pickFile">Choose File</Button>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
// Import the file system plugin from Tauri
import { open } from '@tauri-apps/plugin-dialog'
import Button from './ui/button/Button.vue';

const filePath = ref<string | null>(null)

async function pickFile() {
    try {
        // Use Tauri's dialog.open to pick a file and get its path
        const selected = await open({ multiple: false, filters: [{ name: 'SCT', extensions: ['sct'] }] });
        if (selected) {
            filePath.value = Array.isArray(selected) ? selected[0] : selected;
        }
    } catch (e) {
        filePath.value = null;
    }
}
</script>
