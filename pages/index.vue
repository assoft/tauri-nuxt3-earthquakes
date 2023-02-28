<template>
    <div v-if="filtered?.length > 0" class="grid p-2">
        <QuakeItem :has-time-zone="true" v-for="quake in filtered" :key="quake.id" :quake="quake" />
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
import type { IQuake } from "@/types"

const router = useRouter();

const quakes = ref<IQuake[]>([])

const loading = ref<boolean>(false);

const refreshFrequency = useState<number>('refreshFrequency')
const notifyQuakeSize = useState<number>('notifyQuakeSize')
const filterLocation = useState<string>('filterLocation')

const filtered = computed<IQuake[]>(() => {
    return quakes.value
        .filter((quake: IQuake) => quake.location.includes(filterLocation.value))
        .sort((a: IQuake, b: IQuake) => b.eventDate.toLocaleString().localeCompare(a.eventDate.toLocaleString()))
})

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
}, () => (refreshFrequency.value * 60) * 1000)

onMounted(async () => {
    quakes.value = await fecthLatestFiftyEvents();
    resume();
})

watchEffect(() => resume())
onUnmounted(() => pause())
</script>