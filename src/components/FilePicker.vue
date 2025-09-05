<template>
  <form class="space-y-6" @submit.prevent="submitForm">
    <div>
      <label class="block text-sm font-medium mb-1">Pick LFXX.sct</label>
      <div class="flex items-center gap-2">
        <Button type="button" @click="pickFile('lfxx')">Choose File</Button>
        <span class="text-xs text-muted-foreground">{{ lfxxPath || 'No file selected' }}</span>
      </div>
    </div>
    <div>
      <label class="block text-sm font-medium mb-1">Pick AVISO.sct</label>
      <div class="flex items-center gap-2">
        <Button type="button" @click="pickFile('aviso')">Choose File</Button>
        <span class="text-xs text-muted-foreground">{{ avisoPath || 'No file selected' }}</span>
      </div>
    </div>
    <div>
      <Button type="submit" :disabled="!lfxxPath || !avisoPath">Install</Button>
    </div>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import Button from './ui/button/Button.vue'
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'vue-sonner'

const lfxxPath = ref<string | null>(null)
const avisoPath = ref<string | null>(null)

async function pickFile(type: 'lfxx' | 'aviso') {
  const selected = await open({ multiple: false, filters: [{ name: 'SCT', extensions: ['sct'] }] })
  if (selected) {
    if (type === 'lfxx') lfxxPath.value = Array.isArray(selected) ? selected[0] : selected
    else avisoPath.value = Array.isArray(selected) ? selected[0] : selected
  }
}

async function submitForm() {
  if (!lfxxPath.value || !avisoPath.value) return
  try {
    await invoke('install_aviso', {
      lfxxPath: lfxxPath.value,
      avisoPath: avisoPath.value
    })
    
    toast.success('AVISO installed successfully!', {
      description: 'You can now use the modified LFXX file.',
    });
  } catch (e) {
    toast.error('Install failed: ' + e);
  }
}
</script>
