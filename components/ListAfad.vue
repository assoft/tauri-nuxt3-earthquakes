<template>
    <div v-if="quakes?.length > 0" class="grid">
        <QuakeItem :has-time-zone="true" v-for="quake in sortedQuakes" :key="quake.id" :quake="quake" />
        <Loading v-if="loading" />
    </div>
</template>
  
<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import type { IQuake } from "@/types"

const quakes = ref<IQuake[]>([])
const intervalID = ref<number | undefined>()

const loading = ref<boolean>(false);

const refreshFrequency = useState<number>('refreshFrequency')
const notifyQuakeSize = useState<number>('notifyQuakeSize')
const activeSource = useState<string>('activeSource');

// @ts-ignore
const sortedQuakes = computed(() => quakes.value.sort((a: IQuake, b: IQuake) => a.eventDate < b.eventDate))

const fetchLatestQuakes = async () => {
    if (activeSource.value !== 'afad') {
        return
    }

    loading.value = true;

    const result = await invoke<string>("get_last_five_events")
    const parsed: IQuake[] = JSON.parse(result) as IQuake[]

    if (quakes.value.length >= 100) {
        quakes.value = [];
    }
    if (quakes.value.length > 0) {
        parsed.map(async (quake) => {
            const filtered = quakes.value.find(x => x.id === quake.id)
            if (!filtered) {
                if (quake.magnitude >= notifyQuakeSize.value) {
                    await pushMessage({ title: `Afad Deprem Merkezi`, body: `${quake.location}: ${quake.magnitude} [${quake.depth}km]` });
                }
                quakes.value.push(quake)
            }
        })
    } else {
        quakes.value = parsed;
    }
    setTimeout(() => loading.value = false, 1800)
}

onMounted(async () => {
    if (activeSource.value === 'afad') {
        const initalData = await fecthLatestFiftyEvents();
        quakes.value = initalData;
        intervalID.value = setInterval(fetchLatestQuakes, refreshFrequency.value * 1000)
    }
})

watchEffect(() => {
    if (activeSource.value === 'afad') {
        intervalID.value = setInterval(fetchLatestQuakes, refreshFrequency.value * 1000)
    }
})

onUnmounted(() => clearInterval(intervalID.value))
</script>