<script setup lang="ts">
import Markdown from "./components/MarkdownEditor.vue";
import { MilkdownProvider } from "@milkdown/vue";
</script>

<template>
  <MilkdownProvider>
    <Markdown />
  </MilkdownProvider>
</template>
