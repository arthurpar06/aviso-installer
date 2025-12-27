<template>
  <div class="h-full flex flex-col gap-6">
    <div class="flex gap-2 items-center justify-between shrink-0 px-1">
      <div class="text-sm font-medium text-muted-foreground flex items-center gap-2">
        <span class="inline-block w-2 h-2 rounded-full bg-green-500"></span>
        Source: https://github.com/arthurpar06/lfxx-aviso
      </div>
      <Button @click="fetchFiles" :disabled="loading" variant="ghost" size="sm" class="h-8">
        <Loader2 v-if="loading" class="w-4 h-4 mr-2 animate-spin" />
        Refresh
      </Button>
    </div>

    <div v-if="error" class="text-red-500 text-sm shrink-0 px-1">{{ error }}</div>

    <div v-if="displayedFiles.length > 0" class="flex-1 overflow-y-auto min-h-[400px] pr-2 custom-scrollbar">
      <div class="grid grid-cols-1 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-4 gap-4 pb-4">
        <div v-for="file in displayedFiles" :key="file.path"
          class="group relative flex flex-col items-center justify-center p-6 rounded-xl bg-muted/30 hover:bg-muted/60 transition-all border border-transparent hover:border-primary/10 cursor-default">
          <div
            class="p-3 rounded-full bg-background shadow-sm mb-3 group-hover:scale-110 transition-transform duration-300">
            <FileCode class="w-6 h-6 text-foreground/70" />
          </div>

          <h3 class="font-medium text-base mb-4 text-center text-foreground/90">{{ file.name }}</h3>

          <Button size="sm" variant="default"
            class="w-full opacity-0 group-hover:opacity-100 transition-opacity duration-200 shadow-md"
            @click="handleInstallClick(file)" :disabled="installing === file.path">
            <Loader2 v-if="installing === file.path" class="w-3 h-3 mr-2 animate-spin" />
            {{ installing === file.path ? 'Installing...' : 'Install' }}
          </Button>
        </div>
      </div>
    </div>
    <div v-else-if="searched && !loading" class="text-center text-muted-foreground text-sm py-12 shrink-0">
      No .sct files found in this repository.
    </div>

    <Dialog :open="showConfirm" @update:open="showConfirm = $event">
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{{ confirmTitle }}</DialogTitle>
          <DialogDescription>
            {{ confirmMessage }}
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <Button v-if="pendingFile" variant="outline" @click="showConfirm = false">Cancel</Button>
          <Button v-if="pendingFile" @click="confirmInstall">Continue Anyway</Button>
          <Button v-else @click="showConfirm = false">Close</Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'vue-sonner'
import Button from './ui/button/Button.vue'
import { Loader2, FileCode } from 'lucide-vue-next'
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog'

const props = defineProps<{
  lfxxPath: string | null,
  lfxxEsePath: string | null
}>()

const repo = 'arthurpar06/lfxx-aviso'
const allFiles = ref<any[]>([])
const loading = ref(false)
const error = ref<string | null>(null)
const searched = ref(false)
const installing = ref<string | null>(null)

// Computed for display
const displayedFiles = computed(() => {
  return allFiles.value.filter(f => f.name.endsWith('.sct'))
})

// Confirmation Dialog State
const showConfirm = ref(false)
const confirmTitle = ref('')
const confirmMessage = ref('')
const pendingFile = ref<any>(null)
const pendingContent = ref<string>('')

async function fetchFiles() {
  loading.value = true
  error.value = null
  allFiles.value = []
  searched.value = true

  try {
    const response = await fetch(`https://api.github.com/repos/${repo}/contents`)
    if (!response.ok) throw new Error('Failed to fetch repository contents')

    const data = await response.json()
    allFiles.value = data.filter((item: any) => item.type === 'file')
  } catch (e: any) {
    error.value = e.message
  } finally {
    loading.value = false
  }
}

async function handleInstallClick(file: any) {
  if (!props.lfxxPath) {
    toast.error('Please select an LFXX.sct file first')
    return
  }

  installing.value = file.path

  try {
    // 1. Fetch content
    const response = await fetch(file.download_url)
    if (!response.ok) throw new Error('Failed to download file content')
    const content = await response.text()

    // 2. Check status (SCT only for now)
    const status = await invoke<string>('check_aviso_installed', {
      lfxxPath: props.lfxxPath,
      avisoContent: content
    })

    if (status === 'PARTIALLY_INSTALLED') {
      confirmTitle.value = 'Already Installed (Partial or Full)'
      confirmMessage.value = `The package '${file.name}' appears to be at least partially installed. Please start with a clean LFXX.sct file.`
      pendingFile.value = null
      showConfirm.value = true
      installing.value = null
    } else {
      await performInstall(content, file)
    }

  } catch (e: any) {
    console.error(e);
    toast.error(`Error checking installation: ${e.message}`)
    installing.value = null
  }
}

async function confirmInstall() {
  showConfirm.value = false
  if (pendingFile.value && pendingContent.value) {
    // Unused currently as we block install
  }
}

async function performInstall(content: string, file: any) {
  try {
    // Determine if we should look for ESE
    // Parse ICAO from filename: LFFF_AVISO.sct -> LFFF
    let eseContent: string | null = null;

    // Attempt to find matching ESE
    // Assumption: Filename is starts with ICAO, or we check for exact name match (replacing .sct with .ese)
    const exactEseName = file.name.replace('.sct', '.ese');

    // Try to extract ICAO. Format: ICAO_...
    const parts = file.name.split('_');
    const icaoEseName = parts.length > 0 ? `${parts[0]}.ese` : null;

    let targetEseFile = allFiles.value.find(f => f.name === exactEseName);
    if (!targetEseFile && icaoEseName) {
      targetEseFile = allFiles.value.find(f => f.name === icaoEseName);
    }

    if (targetEseFile) {
      // Fetch ESE content
      try {
        const resp = await fetch(targetEseFile.download_url);
        if (resp.ok) {
          eseContent = await resp.text();
        }
      } catch (e) {
        console.warn("Failed to fetch ESE content", e);
      }
    }

    await invoke('install_aviso_package', {
      lfxxPath: props.lfxxPath,
      lfxxEsePath: props.lfxxEsePath,
      avisoContent: content,
      eseContent: eseContent,
      filename: file.name
    })

    let successMsg = `Installed ${file.name} successfully!`;
    if (eseContent && props.lfxxEsePath) {
      successMsg += ` (and ${targetEseFile?.name})`;
    } else if (targetEseFile && !props.lfxxEsePath) {
      successMsg += ` (Note: ESE file found but no local LFXX.ese selected)`;
    }

    toast.success(successMsg)
  } catch (e: any) {
    console.error(e)
    toast.error(`Failed to install: ${e.message}`)
  } finally {
    installing.value = null
    pendingFile.value = null
    pendingContent.value = ''
  }
}

onMounted(() => {
  fetchFiles()
})
</script>
