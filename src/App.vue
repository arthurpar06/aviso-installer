<template>
  <div class="flex min-h-screen items-center justify-center bg-background p-4">
    <Card class="w-full max-w-4xl">
      <CardHeader class="flex flex-row items-center justify-between">
        <CardTitle class="text-2xl font-bold"
          >EuroScope Aviso Installer by Lun'Air</CardTitle
        >
        <Button
          variant="outline"
          size="sm"
          as="a"
          href="https://github.com/arthurpar06/lfxx-aviso"
          target="_blank"
          class="hidden md:flex"
        >
          <Github class="w-4 h-4 mr-2" />
          Contribute
        </Button>
      </CardHeader>
      <CardContent class="space-y-6">
        <p class="text-muted-foreground text-justify text-sm">
          This tool will copy the content of installed AVISO sections into your
          LFXX file. Ensure you have a blank original LFXX file (without AVISO)
          before installing.
        </p>

        <!-- Common LFXX Picker -->
        <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
          <div class="p-4 border rounded-lg bg-muted/20">
            <label class="block text-sm font-medium mb-2"
              >Target LFXX.sct File</label
            >
            <div class="flex items-center gap-2">
              <Button type="button" variant="outline" @click="pickLfxx"
                >Select LFXX.sct</Button
              >
              <span class="text-xs text-muted-foreground break-all">{{
                lfxxPath || 'No file selected'
              }}</span>
            </div>
          </div>

          <div class="p-4 border rounded-lg bg-muted/20">
            <label class="block text-sm font-medium mb-2"
              >Target LFXX.ese File (Optional)</label
            >
            <div class="flex items-center gap-2">
              <Button type="button" variant="outline" @click="pickLfxxEse"
                >Select LFXX.ese</Button
              >
              <span class="text-xs text-muted-foreground break-all">{{
                lfxxEsePath || 'No file selected'
              }}</span>
            </div>
          </div>
        </div>

        <!-- Mode Switcher -->
        <div class="flex border-b">
          <button
            class="flex-1 py-2 text-sm font-medium border-b-2 transition-colors"
            :class="
              mode === 'local'
                ? 'border-primary text-primary'
                : 'border-transparent text-muted-foreground hover:text-foreground'
            "
            @click="mode = 'local'"
          >
            Local File
          </button>
          <button
            class="flex-1 py-2 text-sm font-medium border-b-2 transition-colors"
            :class="
              mode === 'github'
                ? 'border-primary text-primary'
                : 'border-transparent text-muted-foreground hover:text-foreground'
            "
            @click="mode = 'github'"
          >
            GitHub
          </button>
        </div>

        <!-- Content -->
        <div v-if="mode === 'local'">
          <FilePicker :lfxxPath="lfxxPath" />
        </div>
        <div v-else>
          <GitHubPicker :lfxxPath="lfxxPath" :lfxxEsePath="lfxxEsePath" />
        </div>
      </CardContent>
    </Card>
    <Toaster />
    <Updater />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { open } from '@tauri-apps/plugin-dialog';
import { Github } from 'lucide-vue-next';
import FilePicker from '@/components/FilePicker.vue';
import GitHubPicker from '@/components/GitHubPicker.vue';
import Updater from '@/components/Updater.vue';
import { Card, CardHeader, CardTitle, CardContent } from '@/components/ui/card';
import { Toaster } from 'vue-sonner';
import Button from '@/components/ui/button/Button.vue';
import 'vue-sonner/style.css';

const lfxxPath = ref<string | null>(null);
const lfxxEsePath = ref<string | null>(null);
const mode = ref<'local' | 'github'>('github');

async function pickLfxx() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'SCT', extensions: ['sct'] }],
  });
  if (selected) {
    lfxxPath.value = Array.isArray(selected) ? selected[0] : selected;
  }
}

async function pickLfxxEse() {
  const selected = await open({
    multiple: false,
    filters: [{ name: 'ESE', extensions: ['ese'] }],
  });
  if (selected) {
    lfxxEsePath.value = Array.isArray(selected) ? selected[0] : selected;
  }
}
</script>
