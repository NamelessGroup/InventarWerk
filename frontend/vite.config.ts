import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from "@tailwindcss/vite";

// Avoid importing browser-only modules at top-level. Some dev-only packages
// (like `vite-plugin-vue-devtools`) pull in `@vue/devtools-kit` which expects
// `localStorage` and will throw when the Vite config is evaluated in Node.
// Export a config factory so we can dynamically import the plugin only when
// running the dev server, and stub `localStorage` in Node to be safe.
export default defineConfig(async ({ command }) => {
  const plugins: any[] = [vue()]

  // If a module expects localStorage during import, provide a minimal stub
  // while running in Node so imports don't crash. This stub is safe and
  // only used during the config evaluation phase.
  if (typeof globalThis.localStorage === 'undefined') {
    ;(globalThis as any).localStorage = {
      getItem: (_: string) => null,
      setItem: (_: string, __: string) => {},
      removeItem: (_: string) => {},
      clear: () => {},
    }
  }

  if (command === 'serve') {
    try {
      // Dynamically import the devtools plugin only for the dev server.
      const mod = await import('vite-plugin-vue-devtools')
      const vueDevTools = (mod && (mod as any).default) || mod
      plugins.push(vueDevTools())
    } catch (e) {
      // If the plugin cannot be loaded (missing package), continue without it.
      // This keeps the dev server usable even if the devtools plugin isn't present.
      // eslint-disable-next-line no-console
      console.warn('vite-plugin-vue-devtools not loaded:', (e as any).message || e)
    }
  }

  plugins.push(tailwindcss())

  return {
    plugins,
    resolve: {
      alias: {
        '@': fileURLToPath(new URL('./src', import.meta.url))
      },
    },
  }
})
