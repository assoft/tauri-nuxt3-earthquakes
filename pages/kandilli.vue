<template>
    <div class="grid">
        <template v-if="quakes?.length > 0">
            <QuakeItem :has-time-zone="false" v-for="quake in sortedQuakes" :key="quake.id" :quake="quake" />
        </template>
        <Loading v-if="loading" />
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
const router = useRouter();

// @ts-ignore
const sortedQuakes = computed(() => quakes.value.sort((a: PIQuake, b: PIQuake) => a.eventDate < b.eventDate))

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