// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  ssr: false,
  modules: ['@unocss/nuxt', '@vueuse/nuxt'],
  css: ['@unocss/reset/tailwind-compat.css'],
})
