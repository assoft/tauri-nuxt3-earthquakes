<template>
    <div>
        <div class="flex rounded bg-dark-4/50 hover:bg-dark-4/20 ease-in-out duration-150 mb-1 border-white/15 border-1 border-solid relative overflow-hidden"
             :class="{
                 'bg-red-6': props.quake.location?.includes('Bingöl') || props.quake.location?.includes('BINGOL')
             }">
            <div class="flex-1 grid py-2 px-3 gap-1 pr-16 sm:pr-0">
                <div class="text-white text-sm">{{ renderQuake.location }}</div>
                <div class="inline-flex items-center gap-2 text-sm">
                    <div class="font-medium text-white/60" v-text="renderQuake.eventDate"></div>
                    <div class="bg-amber-400 rounded px-1" v-text="renderQuake.depth"></div>
                </div>
            </div>
            <div class="absolute right-0 inset-y-0 grid place-items-center w-16" :class="[`color-${color}`]">
                <div class="text-lg text-white mx-4" v-text="renderQuake.magnitude"></div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import type { IQuake } from '~~/types';
import { DateTime } from "luxon";

interface IProps {
    quake: Partial<IQuake>
    hasTimeZone: boolean
}

const props = defineProps<IProps>()

const renderQuake = computed(() => {
    return {
        location: props.quake.location,
        magnitude: props.quake.magnitude,
        depth: props.quake.depth?.toFixed(0).toString() + ' km',
        eventDate: props.hasTimeZone ? DateTime.fromISO(String(props.quake.eventDate), { setZone: true, locale: "tr-TR", zone: "utc" }).setZone("utc+3").toISOTime({ includeOffset: false, suppressMilliseconds: true }) : String(props.quake.eventDate).split(" ")[1]
    }
})

const color = computed(() => {
    if (Number(props.quake.magnitude) <= 2.9)
        return 'blue'
    if (Number(props.quake.magnitude) > 2.9)
        return 'green'
    if (Number(props.quake.magnitude) > 3.9)
        return 'yellow'
    if (Number(props.quake.magnitude) > 4.9)
        return 'orange'
    if (Number(props.quake.magnitude) > 5.9)
        return 'red'
    return 'gray'
})
</script>

<style scoped>
.color-blue {
    background-color: #2d9cdb;
}

.color-green {
    background-color: #27AE60;
}

.color-yellow {
    background-color: #F2C94C;
}

.color-orange {
    background-color: #F2994A
}

.color-red {
    background-color: #EB5757
}
</style>