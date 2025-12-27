import { createApp } from 'vue';
import './assets/style.css';
import App from './App.vue';

// Apply dark mode based on OS preference
function applyTheme() {
  const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
  document.documentElement.classList.toggle('dark', prefersDark);
}
applyTheme();
window
  .matchMedia('(prefers-color-scheme: dark)')
  .addEventListener('change', applyTheme);

createApp(App).mount('#app');
