<template>
  <Body class="dark font-sans relative">
    <div style="padding: 4px; height: calc(100vh - 12px);" data-tauri-drag-region class="select-none">
      <div class="pb-16">
        <NuxtPage />
      </div>

      <div class="fixed bottom-0 inset-x-0 select-none">
        <div class="flex items-center justify-between bg-dark-3 p-2 dark:text-white select-none">

          <div class="flex items-center justify-center gap-2 ml-2">
            <NuxtLink class="decoration-none text-white px-1.5 py-1 rounded text-sm" exact-active-class="bg-white/20" to="/">Afad</NuxtLink>
            <NuxtLink class="decoration-none text-white px-1.5 py-1 rounded text-sm" exact-active-class="bg-white/20" to="/kandilli">Kandilli</NuxtLink>
          </div>

          <div class="inline-flex items-center gap-1">
            <div class="text-xs">Bildir</div>
            <select class="bg-dark appearance-none text-white border-none p-2 text-sm" @change="setNotifyQuakeSize" value="3" title="Deprem Bildirim Seçenekleri">
              <option v-for="size in [2, 3, 4, 5, 6]" :value="size" :key="`size_${size}`"> >= {{ size }}</option>
            </select>
          </div>

          <div class="inline-flex items-center gap-1">
            <div class="text-xs">Yenileme Sıklığı</div>
            <select class="bg-dark appearance-none text-white border-none p-2 text-sm" @change="setRefreshFrequency" value="5">
              <option v-for="frequency in frequencies" :value="frequency" :key="`frequency_${frequency}`">{{ frequency }} dk</option>
            </select>
          </div>

          <div>
            <button @click="async () => await invoke('exit_request')"
                    class="bg-transparent border-none outline-none text-white hover:bg-dark-1 ease-in-out duration-200 p-1 rounded-lg cursor-pointer shadow-transparent hover:shadow-cyan hover:shadow active:shadow-sm active:text-cyan">
              <IconsPowerIcon class="w-6 h-6" />
            </button>
          </div>
        </div>
      </div>
    </div>
  </Body>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';

// const source = useState<string>('activeSource', () => 'afad');
const refreshFrequency = useState<number>('refreshFrequency', () => 5);
const notifyQuakeSize = useState<number>('notifyQuakeSize', () => 3);
const frequencies = ref<number[]>([1, 2, 3, 4, 5, 10, 30])

// @ts-ignore
const setRefreshFrequency = (event: Event) => refreshFrequency.value = event.target.value
// @ts-ignore
const setNotifyQuakeSize = (event: Event) => notifyQuakeSize.value = event.target.value

onNuxtReady(() => {
  if (window.location.protocol !== 'tauri:') {
    return
  }
  window.addEventListener('contextmenu', e => {
    e.preventDefault();
    return false;
  }, { capture: true })
})

// onMounted(() => {})
</script>

<style>
@media (prefers-color-scheme: dark) {
  body {
    background: rgba(255, 255, 255, .05);
  }
}

@media (prefers-color-scheme: light) {
  body {
    background: rgba(0, 0, 0, .7);
  }
}
</style>