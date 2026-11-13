<template>
  <div
    class="border-accent bg-primary-dark flex flex-col gap-2 overflow-hidden rounded-sm border-2 p-2"
  >
    <div class="flex items-center gap-2 overflow-hidden">
      <DiscordImage :user="creator" class="h-6" />
      <div
        ref="nameInput"
        class="bold break-wrap min-w-8 border-none bg-transparent pr-5 text-xl outline-hidden"
        :contenteditable="container.ownerUuid === accountData().uuid"
        @blur="updateName()"
        @keydown="
          (e) => {
            if (e.key === 'Enter') {
              e.preventDefault()
              e.stopPropagation()
              updateName()
            }
          }
        "
      >
        {{ container.name }}
      </div>
      <button
        v-if="container.ownerUuid === accountData().uuid"
        class="border-accent bg-primary-default h-7 w-7 shrink-0 rounded-sm border"
        @click="editName()"
      >
        <FontAwesomeIcon :icon="faPen" />
      </button>
      <slot name="extraInfo" />
      <div class="flex-1"><!-- Spacer --></div>
      <button
        class="border-accent bg-primary-default h-7 w-7 shrink-0 rounded-sm border"
        @click="expanded = !expanded"
      >
        <FontAwesomeIcon
          class="transition"
          :class="{ 'rotate-180': !expanded }"
          :icon="faChevronUp"
        />
      </button>
      <slot name="extraButtons" />
      <TimedConfirmationButton
        v-if="container.ownerUuid === accountData().uuid"
        class="border-accent bg-primary-default text-delete h-7 shrink-0 rounded-sm border px-1.5"
        :skip-confirmation="Settings.getInstance().noDeleteConfirmation"
        @confirm="emit('delete')"
      >
        <FontAwesomeIcon :icon="faTrashCan" />

        <template #confirmation> Confirm </template>
      </TimedConfirmationButton>
    </div>

    <CollapseTransition with-opacity speed="0.5s">
      <div v-if="expanded" class="flex flex-col gap-2">
        <slot name="preItems" />

        <div class="flex flex-col gap-2">
          <DraggableContainer
            :model-value="localDraggableItems"
            group="items"
            :item-key="dragKey"
            :animation="150"
            @change="onDragEvent"
          >
            <div v-for="(item, idx) in localDraggableItems" :key="idx" :item="item">
              <slot name="item" :item="item" />
            </div>
          </DraggableContainer>
        </div>

        <slot name="postItems" />
      </div>
    </CollapseTransition>
  </div>
</template>

<script setup lang="ts" generic="T extends { ownerUuid: string; name: string; uuid: string }, I">
import { accountData } from '@/store'
import DiscordImage from './DiscordImage.vue'
import { computed, ref, type PropType } from 'vue'
import { faChevronUp, faPen, faTrashCan } from '@fortawesome/free-solid-svg-icons'
import { Settings } from '@/store/Settings'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import TimedConfirmationButton from './TimedConfirmationButton.vue'
import CollapseTransition from './CollapseTransition.vue'
import type { DraggableItem } from '@/model/Drag.ts'
import { VueDraggableNext as DraggableContainer, type DragChangeEvent } from 'vue-draggable-next'

const props = defineProps({
  container: {
    type: Object as PropType<T>,
    required: true
  },
  items: {
    type: Array<I>,
    required: true
  },
  dragKey: {
    type: String,
    required: true
  }
})

const emit = defineEmits(['updateName', 'delete', 'move'])

const nameInput = ref<HTMLDivElement | null>(null)
const expanded = ref(true)
const creator = computed(
  () =>
    accountData().accounts.filter((account) => account.uuid === props.container.ownerUuid)[0] ?? {
      name: 'Unknown',
      avatar: null,
      dm: false,
      uuid: ''
    }
)
const localDraggableItems = computed(() =>
  props.items.map((i) => ({ ...i, sourceContainer: props.container.uuid }))
)

function updateName() {
  const newName = nameInput.value?.innerText ?? ''
  if (newName.length == 0) {
    return
  }
  if (newName == props.container.name) {
    return
  }
  emit('updateName', newName)
}

function editName() {
  if (nameInput.value) {
    nameInput.value.focus()
  }
}

function onDragEvent(e: DragChangeEvent<DraggableItem<T>>) {
  if (e.added) {
    // Item moved to this inventory
    emit('move', e.added.element, e.added.newIndex, true)
  }

  if (e.moved) {
    // Item moved in this inventory
    emit('move', e.moved.element, e.moved.newIndex, false)
  }

  // We purposefully ignore the `removed` option, since removal will be handled by addition to the new inventory
}
</script>
