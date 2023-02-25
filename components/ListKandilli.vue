<template>
    <div v-if="quakes?.length > 0" class="grid">
        <QuakeItem :has-time-zone="false" v-for="quake in sortedQuakes" :key="quake.id" :quake="quake" />
        <Loading v-if="loading" />
    </div>
</template>
  
<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import type { PIQuake } from "@/types"

const quakes = ref<PIQuake[]>([])
const intervalID = ref<number | undefined>()

const loading = ref<boolean>(false);

const refreshFrequency = useState<number>('refreshFrequency')
const notifyQuakeSize = useState<number>('notifyQuakeSize')
const activeSource = useState<string>('activeSource')

// @ts-ignore
const sortedQuakes = computed(() => quakes.value.sort((a: PIQuake, b: PIQuake) => a.eventDate < b.eventDate))

const fetchLatestQuakes = async () => {
    if (activeSource.value !== 'kandilli') {
        return
    }

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

onMounted(async () => {
    if (activeSource.value === 'kandilli') {
        loading.value = true;
        await fetchLatestQuakes();
        loading.value = false;

        intervalID.value = setInterval(fetchLatestQuakes, refreshFrequency.value * 1000)
    }
})

watchEffect(() => {
    if (activeSource.value === 'kandilli') {
        intervalID.value = setInterval(fetchLatestQuakes, refreshFrequency.value * 1000)
    }
})

onUnmounted(() => clearInterval(intervalID.value))
</script>