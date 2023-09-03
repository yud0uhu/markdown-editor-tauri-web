<script setup lang="ts">
import Markdown from "./components/MarkdownEditor.vue";
import { useTheme } from "vuetify";

const theme = useTheme();

function toggleTheme() {
  theme.global.name.value = theme.global.current.value.dark ? "light" : "dark";
}
</script>

<template>
  <v-app>
    <v-btn @click="toggleTheme">toggle theme</v-btn>
    <Markdown />
  </v-app>
</template>
