/** @type {import('vite').UserConfig} */
export default {
    base: '/chippy/',
    esbuild: {
        supported: {
            'top-level-await': true
        }
    }
}
