<template>
  <Body class="dark font-sans relative">
    <div style="padding: 4px; height: calc(100vh - 12px);" data-tauri-drag-region class="select-none">
      <div class="pb-28">
        <NuxtPage />
      </div>

      <div class="fixed bottom-0 inset-x-0 select-none">
        <div class="flex flex-col gap-2 bg-dark-3 px-4 py-2 dark:text-white">
          <div class="mt-1 inline-flex items-center gap-2 w-full">
            <input type="text"
                   placeholder="Konumlarda ara"
                   @keyup="($event) => filterLocation = $event.target?.value"
                   class="w-full bg-transparent border-white/20 focus:border-white/40 outline-none focus:ring-3 focus:ring-[#224d9b] focus:outline-none focus:border-transparent focus-visible:border-red border-1 py-1.5 px-2 rounded text-sm leading-2 text-white placeholder:text-white/20">
          </div>

          <div class="flex items-center justify-start w-full select-none gap-4">

            <div class="flex items-center justify-center gap-2">
              <NuxtLink class="select-none decoration-none text-white px-1.5 py-1 rounded text-sm" exact-active-class="bg-white/20" to="/">Afad</NuxtLink>
              <NuxtLink class="select-none decoration-none text-white px-1.5 py-1 rounded text-sm" exact-active-class="bg-white/20" to="/kandilli">Kandilli
              </NuxtLink>
            </div>

            <div class="inline-flex items-center gap-2">
              <div class="text-xs">Bildir</div>
              <select class="bg-dark appearance-none text-white border-none p-2 text-sm" @change="() => setNotifyQuakeSize" value="3"
                      title="Deprem Bildirim Seçenekleri">
                <option v-for="size in [2, 3, 4, 5, 6]" :value="size" :key="`size_${size}`"> >= {{ size }}</option>
              </select>
            </div>

            <div class="inline-flex items-center gap-2">
              <div class="text-xs">Yenileme Sıklığı</div>
              <select class="bg-dark appearance-none text-white border-none p-2 text-sm" @change="() => setRefreshFrequency" value="5">
                <option v-for="frequency in frequencies" :value="frequency" :key="`frequency_${frequency}`">{{ frequency }} dk</option>
              </select>
            </div>

            <div class="grow inline-flex justify-end">
              <button @click="async () => await invoke('exit_request')"
                      class="bg-transparent border-none outline-none focus-ring-3 focus:ring-[#224d9b] text-white hover:bg-dark-1 ease-in-out duration-200 p-1 rounded-lg cursor-pointer shadow-transparent hover:shadow-cyan hover:shadow active:shadow-sm active:text-cyan">
                <IconsPowerIcon class="w-6 h-6" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  </Body>
</template>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api';

const refreshFrequency = useState<number>('refreshFrequency', () => 5);
const notifyQuakeSize = useState<number>('notifyQuakeSize', () => 3);
const filterLocation = useState<string>('filterLocation', () => "")

const frequencies = ref<number[]>([1, 2, 3, 4, 5, 10, 30])

const setRefreshFrequency = (event: Event & { target: HTMLInputElement }) => refreshFrequency.value = Number(event.target.value)
const setNotifyQuakeSize = (event: Event & { target: HTMLInputElement }) => notifyQuakeSize.value = Number(event.target.value)

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