<template>
  <PopUp @close="close()">
    <div class="flex items-center justify-center">
      <div
        class="grid-rows-[repeat(5, auto)] grid h-fit w-fit gap-5 md:grid-cols-2 md:grid-rows-[repeat(3,auto)] md:gap-x-10"
      >
        <div class="md:min-w-100">
          <span>spell-XX.json</span>
          <button
            class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
            @click="loadSpellJson()"
          >
            Add
          </button>
          <span class="text-accent">{{ spellFileNameText }}</span>
        </div>
        <div>
          <span>sources.json</span>
          <button
            class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
            @click="loadSourcesJson()"
          >
            Set
          </button>
          <span v-if="sources !== undefined" class="text-accent">uploaded</span>
        </div>

        <button
          class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
          @click="upload()"
        >
          Upload
        </button>
        <button
          class="border-accent bg-primary-default w-full rounded-sm border p-1 outline-hidden"
          @click="close()"
        >
          Cancel
        </button>
        <span class="text-error-text md:col-span-2">{{ errorText }}</span>
      </div>
    </div>
  </PopUp>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'
import PopUp from '../PopUp.vue'
import { type SpellJson, type SourceJson, parseSpellList } from '@/parser/spellParser.ts'

const emit = defineEmits(['close'])

const sources = ref<SourceJson | undefined>()
const spells = ref<SpellJson[]>([])
const spellFileNames = ref<string[]>([])
const errorText = ref('')

const spellFileNameText = computed(() => {
  if (spellFileNames.value.length > 6) {
    return `${spellFileNames.value.slice(0, 4).join(', ')}, ... (+${spellFileNames.value.length - 4} more)`
  }
  return spellFileNames.value.join(', ')
})

function close() {
  sources.value = undefined
  spells.value = []
  errorText.value = ''
  emit('close')
}

async function upload() {
  if (!sources.value) {
    errorText.value = 'No sources file given'
    return
  }
  if (spells.value.length === 0) {
    errorText.value = 'No spell file given'
    return
  }
  await parseSpellList(spells.value, sources.value)
  close()
}

interface FileResult<T> {
  content: T
  fileName: string
}

async function loadSourcesJson() {
  const result = await openFileChooser<SourceJson>(false)
  if (result.length === 0) return
  sources.value = result[0].content
}

async function loadSpellJson() {
  const result = await openFileChooser<SpellJson>(true)
  spellFileNames.value.push(...result.map((r) => r.fileName))
  spells.value.push(...result.map((r) => r.content))
}

function openFileChooser<T>(multiple: boolean): Promise<FileResult<T>[]> {
  return new Promise((resolve, reject) => {
    const input = document.createElement('input')
    input.type = 'file'
    input.accept = '.json,application/json'
    input.multiple = multiple

    input.addEventListener('change', async () => {
      const files = Array.from(input.files ?? [])
      try {
        const results = await Promise.all(
          files.map(async (file): Promise<FileResult<T>> => {
            const text = await file.text()
            try {
              return { content: JSON.parse(text) as T, fileName: file.name }
            } catch (error) {
              throw new Error(`Failed to parse "${file.name}" as JSON: ${error}`, { cause: error })
            }
          })
        )
        resolve(results)
      } catch (error) {
        reject(error)
      }
    })

    input.addEventListener('cancel', () => resolve([]))
    input.click()
  })
}
</script>
