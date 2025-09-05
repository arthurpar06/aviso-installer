<template>
	<div class="flex min-h-screen items-center justify-center bg-background">
		<Card class="w-full max-w-lg">
			<CardHeader>
				<CardTitle class="text-center text-3xl">Aviso Installer</CardTitle>
			</CardHeader>
			<CardContent>
				<p class="text-muted-foreground text-justify mb-4 text-sm">
					This tool will copy the content of each section from your AVISO file into your LFXX file. You need
					to make sure that the sections are correctly defined in the AVISO file. Please note that this tool
					will simply append the content of the AVISO to the LFXX file. There is no logic for merging or
					updating, so you cannot update an installed AVISO. Instead, you must use a blank original LFXX file
					(without the AVISO) and then install it.
				</p>
				<FilePicker />
			</CardContent>
		</Card>
		<Toaster />
	</div>
</template>


<script setup lang="ts">
import FilePicker from '@/components/FilePicker.vue'
import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card";
import { Toaster } from '@/components/ui/sonner'
import 'vue-sonner/style.css'
</script>
