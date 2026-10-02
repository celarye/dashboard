<script setup lang="ts">
import AppSidebar from '@/components/AppSidebar.vue'
</script>

<template>
  <AppSidebar>
    <main class="px-4">
      <div class="flex flex-1 flex-col gap-4 py-4">
        <h1>Services</h1>
        <div class="grid auto-rows-min gap-4 md:grid-cols-3" v-for="_ in 3">
          <span v-for="_ in 3">
            <div class="aspect-video rounded-xl bg-muted/50" />
          </span>
        </div>
        <div class="min-h-[100vh] flex-1 rounded-xl bg-muted/50 md:min-h-min" />
      </div>
      <p>frontend version: unknown</p>
      <p>backend version: unknown</p>
    </main>
  </AppSidebar>
</template>

<style scoped></style>
