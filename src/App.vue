
<template>
	<main class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-200">
		<section class="w-full max-w-lg bg-white rounded-xl shadow-lg p-8 space-y-8">
			<h1 class="text-3xl font-bold text-center text-gray-900 mb-6">Aviso Installer</h1>
			<FilePicker />
		</section>
	</main>
</template>

<script setup lang="ts">
import FilePicker from './components/FilePicker.vue'
</script>
