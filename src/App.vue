<template>
  <v-app>
    <Header
      :isEditorFullScreen="isEditorFullScreen"
      :isPreviewFullScreen="isPreviewFullScreen"
      @update:isEditorFullScreen="onUpdateIsEditorFullScreen"
      @update:isPreviewFullScreen="onUpdateIsPreviewFullScreen"
    />
    <MarkdownEditor
      :isEditorFullScreen="isEditorFullScreen"
      :isPreviewFullScreen="isPreviewFullScreen"
    />
  </v-app>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import MarkdownEditor from "./components/MarkdownEditor.vue";

import { ref } from "vue";
const isEditorFullScreen = ref(false);
const isPreviewFullScreen = ref(false);

const onUpdateIsEditorFullScreen = (value: boolean) => {
  isEditorFullScreen.value = value;
};

const onUpdateIsPreviewFullScreen = (value: boolean) => {
  isPreviewFullScreen.value = value;
};
</script>
