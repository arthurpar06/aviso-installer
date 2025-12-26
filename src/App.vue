<template>
  <div class="flex min-h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-4xl">
      <CardHeader>
        <CardTitle class="text-center text-3xl">EuroScope Aviso Installer by Lun'Air</CardTitle>
      </CardHeader>
      <CardContent class="space-y-6">
        <p class="text-muted-foreground text-justify text-sm">
          This tool will copy the content of installed AVISO sections into your LFXX file.
          Ensure you have a blank original LFXX file (without AVISO) before installing.
        </p>

        <!-- Common LFXX Picker -->
        <div class="p-4 border rounded-lg bg-muted/20">
          <label class="block text-sm font-medium mb-2">Target LFXX.sct File</label>
          <div class="flex items-center gap-2">
            <Button type="button" variant="outline" @click="pickLfxx">Select LFXX File</Button>
            <span class="text-xs text-muted-foreground break-all">{{ lfxxPath || 'No file selected' }}</span>
          </div>
        </div>

        <!-- Mode Switcher -->
        <div class="flex border-b">
          <button @click="mode = 'local'" class="flex-1 py-2 text-sm font-medium border-b-2 transition-colors"
            :class="mode === 'local' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'">
            Local File
          </button>
          <button @click="mode = 'github'" class="flex-1 py-2 text-sm font-medium border-b-2 transition-colors"
            :class="mode === 'github' ? 'border-primary text-primary' : 'border-transparent text-muted-foreground hover:text-foreground'">
            GitHub
          </button>
        </div>

        <!-- Content -->
        <div v-if="mode === 'local'">
          <FilePicker :lfxxPath="lfxxPath" />
        </div>
        <div v-else>
          <GitHubPicker :lfxxPath="lfxxPath" />
        </div>

      </CardContent>
    </Card>
    <Toaster />
  </div>
</template>


<script setup lang="ts">
import { ref } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import FilePicker from '@/components/FilePicker.vue'
import GitHubPicker from '@/components/GitHubPicker.vue'
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Toaster } from '@/components/ui/sonner'
import Button from '@/components/ui/button/Button.vue' // Assume Button exists there or use standard
import 'vue-sonner/style.css'

const lfxxPath = ref<string | null>(null)
const mode = ref<'local' | 'github'>('github') // Default to github as requested features
// Actually user said "Pick an AVISO sct to install it... now discover list from github"
// So having both options is good.

async function pickLfxx() {
  const selected = await open({ multiple: false, filters: [{ name: 'SCT', extensions: ['sct'] }] })
  if (selected) {
    lfxxPath.value = Array.isArray(selected) ? selected[0] : selected
  }
}
</script>
