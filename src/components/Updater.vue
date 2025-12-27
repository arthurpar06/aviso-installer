<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import Button from '@/components/ui/button/Button.vue';
import { toast } from 'vue-sonner';

const updateAvailable = ref(false);
const updateVersion = ref('');
const updateBody = ref('');
const downloading = ref(false);
const downloaded = ref(false);

const updateObj = ref<any>(null);

onMounted(async () => {
  try {
    const update = await check();
    if (update) {
      updateAvailable.value = true;
      updateVersion.value = update.version;
      updateBody.value = update.body || '';
      updateObj.value = update;
    }
  } catch (error) {
    console.error('Failed to check for updates:', error);
  }
});

async function startUpdate() {
  if (!updateObj.value) return;

  downloading.value = true;
  let downloadedBytes = 0;
  let contentLength = 0;

  try {
    await updateObj.value.downloadAndInstall((event: any) => {
      switch (event.event) {
        case 'Started':
          contentLength = event.data.contentLength || 0;
          console.log(`started downloading ${contentLength} bytes`);
          break;
        case 'Progress':
          downloadedBytes += event.data.chunkLength;
          console.log(`downloaded ${downloadedBytes} from ${contentLength}`);
          break;
        case 'Finished':
          console.log('download finished');
          break;
      }
    });

    downloading.value = false;
    downloaded.value = true;

    await relaunch();
  } catch (error) {
    console.error('Update failed:', error);
    downloading.value = false;
    toast.error('Update failed: ' + (error as Error).message);
  }
}
</script>

<template>
  <Dialog
    :open="updateAvailable"
    @update:open="(val) => !val && (updateAvailable = false)"
  >
    <DialogContent>
      <DialogHeader>
        <DialogTitle>Update Available</DialogTitle>
        <DialogDescription>
          A new version of Aviso Installer ({{ updateVersion }}) is available.
          <div
            v-if="updateBody"
            class="mt-2 text-sm text-muted-foreground whitespace-pre-wrap max-h-40 overflow-y-auto bg-muted/50 p-2 rounded"
          >
            {{ updateBody }}
          </div>
        </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <Button
          variant="outline"
          :disabled="downloading"
          @click="updateAvailable = false"
          >Maybe Later</Button
        >
        <Button :disabled="downloading" @click="startUpdate">
          <span v-if="downloading">Downloading...</span>
          <span v-else-if="downloaded">Restarting...</span>
          <span v-else>Update Now</span>
        </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
