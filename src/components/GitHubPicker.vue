<template>
  <div class="h-full flex flex-col gap-6">
    <div class="flex gap-2 items-center justify-between shrink-0 px-1">
      <div class="text-sm font-medium text-muted-foreground flex items-center gap-2">
        <span class="inline-block w-2 h-2 rounded-full bg-green-500"></span>
        Source: arthurpar06/lfxx-aviso
      </div>
      <Button @click="fetchFiles" :disabled="loading" variant="ghost" size="sm" class="h-8">
        <Loader2 v-if="loading" class="w-4 h-4 mr-2 animate-spin" />
        Refresh
      </Button>
    </div>

    <div v-if="error" class="text-red-500 text-sm shrink-0 px-1">{{ error }}</div>

    <div v-if="files.length > 0" class="flex-1 overflow-y-auto min-h-[400px] pr-2 custom-scrollbar">
      <div class="grid grid-cols-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-4 gap-4 pb-4">
        <div v-for="file in files" :key="file.path"
          class="group relative flex flex-col items-center justify-center p-6 rounded-xl bg-muted/30 hover:bg-muted/60 transition-all border border-transparent hover:border-primary/10 cursor-default">
          <div
            class="p-3 rounded-full bg-background shadow-sm mb-3 group-hover:scale-110 transition-transform duration-300">
            <FileCode class="w-6 h-6 text-foreground/70" />
          </div>

          <h3 class="font-medium text-base mb-4 text-center text-foreground/90">{{ file.name }}</h3>

          <Button size="sm" variant="default"
            class="w-full opacity-0 group-hover:opacity-100 transition-opacity duration-200 shadow-md"
            @click="install(file)" :disabled="installing === file.path">
            <Loader2 v-if="installing === file.path" class="w-3 h-3 mr-2 animate-spin" />
            {{ installing === file.path ? 'Installing...' : 'Install' }}
          </Button>
        </div>
      </div>
    </div>
    <div v-else-if="searched && !loading" class="text-center text-muted-foreground text-sm py-12 shrink-0">
      No .sct files found in this repository.
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'vue-sonner'
import Button from './ui/button/Button.vue'
import { Loader2, FileCode } from 'lucide-vue-next'

const props = defineProps<{
  lfxxPath: string | null
}>()

const repo = 'arthurpar06/lfxx-aviso'
const files = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const searched = ref(false)
const installing = ref<string | null>(null)

async function fetchFiles() {
  loading.value = true
  error.value = null
  files.value = []
  searched.value = true

  try {
    const response = await fetch(`https://api.github.com/repos/${repo}/contents`)
    if (!response.ok) throw new Error('Failed to fetch repository contents')

    const data = await response.json()
    files.value = data.filter((item: any) => item.type === 'file' && item.name.endsWith('.sct'))
  } catch (e: any) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

async function install(file: any) {
  if (!props.lfxxPath) {
    toast.error('Please select an LFXX file first')
    return
  }

  installing.value = file.path

  try {
    // Fetch raw content
    const response = await fetch(file.download_url)
    if (!response.ok) throw new Error('Failed to download file content')
    const content = await response.text()

    // Install
    await invoke('install_aviso_content', {
      lfxxPath: props.lfxxPath,
      avisoContent: content
    })

    toast.success(`Installed ${file.name} successfully!`)
  } catch (e: any) {
    toast.error(`Failed to install: ${e.message}`)
  } finally {
    installing.value = null
  }
}

onMounted(() => {
  fetchFiles()
})
</script>
