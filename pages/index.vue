<template>
    <div v-if="quakes?.length > 0" class="grid">
        <QuakeItem :has-time-zone="true" v-for="quake in sortedQuakes" :key="quake.id" :quake="quake" />
        <Loading v-if="loading" />
    </div>
</template>
  
<script setup lang="ts">
import { invoke } from "@tauri-apps/api"
import type { IQuake } from "@/types"

const router = useRouter();

const quakes = ref<IQuake[]>([])

const loading = ref<boolean>(false);

const refreshFrequency = useState<number>('refreshFrequency')
const notifyQuakeSize = useState<number>('notifyQuakeSize')

// @ts-ignore
const sortedQuakes = computed(() => quakes.value.sort((a: IQuake, b: IQuake) => a.eventDate < b.eventDate))

const fetchLatestQuakesFromAfad = async () => {
    if (router.currentRoute.value.name === 'index') {
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
}

const { resume, pause } = useIntervalFn(() => {
    fetchLatestQuakesFromAfad()
}, () => refreshFrequency.value * 1000)

onMounted(async () => {
    quakes.value = await fecthLatestFiftyEvents();
    resume();
})

watchEffect(() => resume())
onUnmounted(() => pause())
</script>