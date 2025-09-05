
<template>
	<main class="min-h-screen flex items-center justify-center bg-gradient-to-br from-gray-50 to-gray-200">
		<section class="w-full max-w-lg bg-white rounded-xl shadow-lg p-8 space-y-8">
			<h1 class="text-3xl font-bold text-center text-gray-900 mb-6">Aviso Installer</h1>
      <p class="text-gray-600 text-sm text-justify">
        This tool will copy the content of each section from your AVISO file into your LFXX file. You need to make sure that the sections are correctly defined in the AVISO file. Please note that this tool will simply append the content of the AVISO to the LFXX file. There is no logic for merging or updating, so you cannot update an installed AVISO. Instead, you must use a blank original LFXX file (without the AVISO) and then install it.
      </p>
			<FilePicker />
		</section>
	</main>
</template>

<script setup lang="ts">
import FilePicker from './components/FilePicker.vue'
</script>
