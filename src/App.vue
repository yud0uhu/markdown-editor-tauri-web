<template>
  <v-app>
    <Header
      :isEditorFullScreen="isEditorFullScreen"
      :isPreviewFullScreen="isPreviewFullScreen"
      :isWindowFullScreen="isWindowFullScreen"
      @update:isEditorFullScreen="onUpdateIsEditorFullScreen"
      @update:isPreviewFullScreen="onUpdateIsPreviewFullScreen"
      @update:onUpdateWindowFullScreen="onUpdateWindowFullScreen"
      @fileOpen="openFile"
      @fileSave="saveFile"
    />
    <MarkdownEditor
      :isEditorFullScreen="isEditorFullScreen"
      :isPreviewFullScreen="isPreviewFullScreen"
      :markdownText="markdownText"
      @update:onMarkdownUpdate="onMarkdownUpdate"
    />
  </v-app>
</template>

<script setup lang="ts">
import Header from "./components/Header.vue";
import MarkdownEditor from "./components/MarkdownEditor.vue";

import { ref } from "vue";
import { open, save } from "@tauri-apps/api/dialog";
import { readTextFile, writeFile } from "@tauri-apps/api/fs";
import { appDir } from "@tauri-apps/api/path";

const isEditorFullScreen = ref(false);
const isPreviewFullScreen = ref(false);
const isWindowFullScreen = ref(false);
const markdownText = ref("");

/**
 * @deprecated appDir(): Promise<string>
 */
const openFile = async () => {
  try {
    const selected = await open({
      directory: false,
      multiple: false,
      defaultPath: await appDir(),
      filters: [{ name: "Markdown and Text Files", extensions: ["md", "txt"] }],
    });

    if (selected !== null) {
      if (!Array.isArray(selected)) {
        const filePath = selected;
        const contents = await readTextFile(filePath);
        console.log("hoge:" + contents);
        onMarkdownUpdate(contents);
      }
    }
  } catch (error) {
    console.error(error);
  }
};

const saveFile = async () => {
  try {
    const result = await save({
      defaultPath: "untitled.txt",
      filters: [{ name: "Markdown and Text Files", extensions: ["md", "txt"] }],
    });

    if (result) {
      const fileContents = markdownText.value;

      await writeFile({
        path: result,
        contents: fileContents ? fileContents : "",
      });
      console.log("以下の内容を保存します" + fileContents);
      onMarkdownUpdate(fileContents ? fileContents : "");
    }
  } catch (error) {
    console.error(error);
  }
};

const onUpdateIsEditorFullScreen = (value: boolean) => {
  isEditorFullScreen.value = value;
};

const onUpdateIsPreviewFullScreen = (value: boolean) => {
  isPreviewFullScreen.value = value;
};

const onUpdateWindowFullScreen = (value: boolean) => {
  isWindowFullScreen.value = value;
};

const emits = defineEmits(["update:markdownText"]);

const onMarkdownUpdate = (value: string) => {
  console.log("Markdownが更新されました:", value);
  markdownText.value = value;
  emits("update:markdownText", value);
};
</script>
