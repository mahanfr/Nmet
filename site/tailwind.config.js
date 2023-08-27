/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{html,js,svelte,ts}'],
  theme: {
    extend: {
        colors:{
            'accent': '#5B9A8B',
            'bg-darker': '#252B48',
            'bg': '#1b1f36',
            'primary': '#445069',
            'secondary': '#FBC252',
        },
    }
  },
  plugins: []
};
