<template>
  <div class="space-y-2 p-2 text-white" :class="errorList.length > 0 ? 'block' : 'hidden'">
    <div
      v-for="error in errorList"
      :key="error.message"
      class="rounded bg-red-800"
      @click="error.timeToLive = 0"
    >
      <p class="p-1"><FontAwesomeIcon :icon="faCircleXmark" /> {{ error.message }}</p>
      <div
        class="h-1 rounded-bl bg-red-400"
        :style="{ width: getBarFillPercentage(error.timeToLive) }"
      ></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { ErrorHandler } from './ErrorHandler'
import { FontAwesomeIcon } from '@fortawesome/vue-fontawesome'
import { faCircleXmark } from '@fortawesome/free-regular-svg-icons'

ErrorHandler.getInstance().addListener(addError)

const errorList = ref<ErrorDisplayElement[]>([])

const tickTime = 10
const maxTime = 3000

const intervalID = ref(-1)

function addError(e: Error) {
  errorList.value.push({
    message: e.message,
    timeToLive: maxTime
  })
  if (errorList.value.length == 1) {
    intervalID.value = buildInterval()
  }
}

function buildInterval() {
  return setInterval(() => {
    errorList.value.forEach((e) => {
      e.timeToLive -= tickTime
    }, tickTime)
    errorList.value = errorList.value.filter((e) => e.timeToLive > 0)
    if (errorList.value.length === 0) {
      clearInterval(intervalID.value)
    }
  }, tickTime)
}

function getBarFillPercentage(timeToLive: number) {
  return `${100 - ((maxTime - timeToLive) / maxTime) * 100}%`
}

interface ErrorDisplayElement {
  message: string
  timeToLive: number
}
</script>
