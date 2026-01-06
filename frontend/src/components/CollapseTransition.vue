<template>
    <Transition name="collapse" @enter="updateFullHeight" @leave="updateFullHeight">
        <slot />
    </Transition>
</template>

<script setup lang="ts">
import { ref } from 'vue';

const fullHeight = ref("100vwh") // We have to guess a sensible initial value here, for the very first animation

function updateFullHeight(el: Element) {
    fullHeight.value = `${el.scrollHeight}px`
}
</script>

<style>
.collapse-enter-from,
.collapse-leave-to {
    max-height: 0px;
    opacity: 0;
}

.collapse-enter-active,
.collapse-leave-active {
    transition-property: max-height opacity;
    transition-duration: 0.5s;
    transition-timing-function: ease;
}

.collapse-enter-to,
.collapse-leave-from {
    max-height: v-bind(fullHeight);
    opacity: 100
}
</style>