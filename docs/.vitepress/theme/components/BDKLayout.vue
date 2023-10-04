<script setup lang="ts">
import DefaultTheme from 'vitepress/theme';

import BDKWalletControl from './BDKWalletControl.vue';

const { Layout } = DefaultTheme;

</script>

<template>
  <Layout>
    <template #nav-bar-content-after>
      <Suspense>
        <BDKWalletControl />
      </Suspense>
    </template>
  </Layout>
</template>
