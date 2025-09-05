<template>
  <div class="space-y-2">
    <div v-for="aviso in avisos" :key="aviso.id" class="flex items-center gap-2">
      <shadcn-button @click="installAviso(aviso)">Install {{ aviso.name }}</shadcn-button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
const avisos = ref([
  { id: 1, name: 'Aviso 1' },
  { id: 2, name: 'Aviso 2' },
  { id: 3, name: 'Aviso 3' }
])

function installAviso(aviso: any) {
  // Dummy install logic
  // Will call Tauri backend later
  alert(`Install ${aviso.name}`)
}
</script>
