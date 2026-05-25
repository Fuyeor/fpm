<!-- @/layout/Bottom.vue -->
<template>
  <div class="bottom-sidebar">
    <router-link :to="{ name: 'Home' }" class="nav-item" active-class="active">
      <img :src="getIconUrl('home')" />
    </router-link>
  </div>
</template>

<script setup lang="ts">
import { getIconUrl } from '@fuyeor/commons';
</script>
