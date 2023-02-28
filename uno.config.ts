import presetUno from '@unocss/preset-uno'
import presetWebFonts from '@unocss/preset-web-fonts'
import presetTypography from '@unocss/preset-typography'
import transformerDirectives from '@unocss/transformer-directives'

import { defineConfig } from 'unocss'

export default defineConfig({
  theme: {
    breakpoints: {
      sm: '640px',
      md: '768px',
      lg: '1024px',
      xl: '1280px',
      '2xl': '1536px',
      '3xl': '1920px',
      // => @media (min-height: 800px) { ... }
    },
  },
  presets: [
    presetUno({
      dark: 'class',
    }),
    presetWebFonts({
      provider: 'google',
      fonts: {
        sans: {
          name: 'Roboto',
          weights: [100, 200, 300, 400, 500, 600, 700, 800, 900],
        },
        mono: {
          name: 'Jetbrains Mono',
          weights: [100, 200, 300, 400, 500, 600, 700, 800, 900],
        },
        poppins: {
          name: 'Poppins',
          weights: [100, 200, 300, 400, 500, 600, 700, 800, 900],
        },
      },
    }),
    presetTypography(),
  ],
  transformers: [transformerDirectives()],
})
