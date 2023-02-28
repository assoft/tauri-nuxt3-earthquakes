<template>
    <div v-if="filtered?.length > 0" class="grid p-2">
        <QuakeItem :has-time-zone="false" v-for="quake in filtered" :key="quake.id" :quake="quake" />
        <Loading v-if="loading" />
    </div>
    <div v-if="quakes.length > 0 && filtered?.length < 1" class="w-full h-[calc(100vh-80px)] grid place-items-center">
        <span class="bg-amber-500 p-2 rounded text-white text-shadow">Arama kriterine uyan kayıt bulunamadı!</span>
    </div>
    <div v-if="quakes.length < 1" class="w-full h-[calc(100vh-80px)] grid place-items-center">
        <span class="text-white text-shadow">Yükleniyor...</span>
    </div>
</template>
  
<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import { useIntervalFn } from '@vueuse/core'

import type { PIQuake } from "@/types"

const quakes = ref<PIQuake[]>([])

const loading = ref<boolean>(false);

const refreshFrequency = useState<number>('refreshFrequency')
const notifyQuakeSize = useState<number>('notifyQuakeSize')
const filterLocation = useState<string>('filterLocation')

const router = useRouter();

const filtered = computed<PIQuake[]>(() => {
    return quakes.value
        .filter((quake: PIQuake) => quake.location.includes(filterLocation.value))
        .sort((a: PIQuake, b: PIQuake) => b.eventDate.toLocaleString().localeCompare(a.eventDate.toLocaleString()))
})

const fetchLatestQuakesFromKandilli = async () => {
    if (router.currentRoute.value.name === 'kandilli') {
        loading.value = true;

        const result = await invoke<string>("fetch_quakes")
        const parsed: PIQuake[] = JSON.parse(result) as PIQuake[]

        const mapped: PIQuake[] = parsed.map((quake: PIQuake) => {
            return {
                id: quake.id,
                eventDate: quake.eventDate,
                magnitude: parseFloat(String(quake.magnitude)),
                location: quake.location,
                depth: parseInt(String(quake.depth))
            }
        })

        if (quakes.value.length > 0) {
            mapped.map(async (quake) => {
                const filtered = quakes.value.find((x: PIQuake) => x.id === quake.id)
                if (!filtered) {
                    if (quake.magnitude >= notifyQuakeSize.value) {
                        await pushMessage({ title: `Kandilli Rasathanesi`, body: `${quake.location}: ${quake.magnitude} [${quake.depth}km]` });
                    }
                    quakes.value.push(quake)
                }
            })
        } else {
            quakes.value = mapped;
        }
        setTimeout(() => loading.value = false, 1700)
    }
}

const { resume, pause } = useIntervalFn(() => {
    fetchLatestQuakesFromKandilli()
}, () => (refreshFrequency.value * 60) * 1000)

onMounted(() => {
    fetchLatestQuakesFromKandilli()
    resume()
})

onUnmounted(() => pause())
</script>