import type { Config } from 'tailwindcss'
import defaultTheme from 'tailwindcss/defaultTheme'

export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {},
        screens: {
            xs: '500px',
            ...defaultTheme.screens
        }
    },
    plugins: []
} satisfies Config
