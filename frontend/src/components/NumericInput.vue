<template>
  <input
    v-model="value"
    type="text"
    :readonly="readonly"
    @input="onInput"
    @blur="onBlur"
    @keydown="
      (e) => {
        if (e.key === 'Enter') {
          onBlur()
        }
      }
    "
  />
</template>

<script setup lang="ts">
import { ErrorHandler } from '@/errorHandling/ErrorHandler'
import { nextTick, ref, watch } from 'vue'

const props = defineProps({
  modelValue: {
    type: Number,
    required: true
  },
  defaultValue: {
    type: Number,
    default: 0
  },
  readonly: {
    type: Boolean,
    required: false,
    default: false
  }
})

const emit = defineEmits(['update:modelValue', 'update'])

const value = ref(props.modelValue.toString())

function onInput() {
  value.value = value.value.replace(/[^0-9+*/()-]/g, '')
}

function onBlur() {
  const result = evaluate(value.value)
  value.value = result.toString()
  emit('update:modelValue', result)
  nextTick(() => emit('update', result))
}

function evaluate(content: string) {
  let val = props.modelValue
  if (content == '') {
    val = props.defaultValue
  } else if (content.match(/^[+-]?\d+$/)) {
    val = parseInt(content)
  } else {
    try {
      val = eval(content)
    } catch (e) {
      ErrorHandler.getInstance().registerError(e as Error)
    }
  }
  return val
}

watch(
  () => props.modelValue,
  (newValue) => {
    value.value = newValue.toString()
  }
)
</script>
