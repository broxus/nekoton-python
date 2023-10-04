<template>
  <div
    class="image-container"
    :style="{ display: 'flex', alignItems: alignment, justifyContent: 'center' }"
  >
    <img :src="src" :alt="altText" :style="{ maxWidth: maxWidth, padding: padding }" />
  </div>
</template>

<script lang="ts">
export default {
  props: {
    src: {
      type: String,
      required: true,
    },
    altText: {
      type: String,
      default: 'Image not available',
    },
    width: {
      type: [Number, String],
      default: 'auto',
    },
    height: {
      type: [Number, String],
      default: 'auto',
    },
    maxWidth: {
      type: String,
      default: '100%',
    },
    padding: {
      type: String,
      default: '0 0 0 0',
    },
    alignment: {
      type: String,
      default: 'center',
    },
  },
};
</script>
