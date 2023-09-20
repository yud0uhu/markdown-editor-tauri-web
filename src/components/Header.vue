<template>
  <v-row class="header" align="end">
    <v-col class="ml-auto">
      <v-toolbar>
        <v-btn @click="toggleFullScreen('editor')" icon>
          <v-icon :color="isEditorFullScreen ? 'grey' : ''">mdi-pencil</v-icon>
        </v-btn>
        <v-btn @click="toggleFullScreen('preview')" icon>
          <v-icon :color="isPreviewFullScreen ? 'grey' : ''">mdi-eye</v-icon>
        </v-btn>
        <v-btn @click="toggleTheme" icon>
          <v-icon>
            {{
              theme.global.current.value.dark
                ? "mdi-lightbulb"
                : "mdi-lightbulb-outline"
            }}
          </v-icon>
        </v-btn>
        <v-btn @click="toggleWindowFullScreen()" icon>
          <v-icon :color="isWindowFullScreen ? 'grey' : ''"
            >mdi-arrow-expand-all</v-icon
          >
        </v-btn>
        <v-spacer></v-spacer>
        <v-btn @click="showDrawer()" icon>
          <v-icon>mdi-menu</v-icon>
        </v-btn>
        <v-navigation-drawer v-model="isShowDrawer" right>
          <v-list>
            <v-list-item @click="fileOpen">
              <v-list-item>
                <v-icon>mdi-folder-open</v-icon>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>File Open</v-list-item-title>
              </v-list-item>
            </v-list-item>

            <v-list-item @click="fileSave">
              <v-list-item>
                <v-icon>mdi-content-save</v-icon>
              </v-list-item>
              <v-list-item>
                <v-list-item-title>File Save</v-list-item-title>
              </v-list-item>
            </v-list-item>
          </v-list>
        </v-navigation-drawer>
      </v-toolbar>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { defineProps } from "vue";
import { useTheme } from "vuetify/lib/framework.mjs";
import { ref } from "vue";
import { appWindow } from "@tauri-apps/api/window";

const props = defineProps({
  isEditorFullScreen: Boolean,
  isPreviewFullScreen: Boolean,
  isWindowFullScreen: Boolean,
  markdownText: String,
});

const emits = defineEmits([
  "update:isEditorFullScreen",
  "update:isPreviewFullScreen",
  "update:isWindowFullScreen",
  "update:markdownUpdate",
  "fileOpen",
  "fileSave",
]);

const isShowDrawer = ref(false);
const theme = useTheme();
const isEditorFullScreen = ref(props.isEditorFullScreen);
const isPreviewFullScreen = ref(props.isPreviewFullScreen);
const isWindowFullScreen = ref(props.isWindowFullScreen);

const showDrawer = async () => {
  isShowDrawer.value = !isShowDrawer.value;
};
const toggleWindowFullScreen = async () => {
  isWindowFullScreen.value = !isWindowFullScreen.value;
  if (isWindowFullScreen.value == false) {
    maximizeWindow();
  }
  if (isWindowFullScreen.value == true) {
    minimizeWindow();
  }
};
const minimizeWindow = async () => {
  await appWindow.center();
  await appWindow.minimize();
  await appWindow.setFocus();
};
const maximizeWindow = async () => {
  await appWindow.center();
  await appWindow.maximize();
  await appWindow.setFocus();
};

const fileOpen = () => {
  emits("fileOpen");
};

const fileSave = () => {
  emits("fileSave");
};

function toggleFullScreen(cardType: string) {
  if (cardType === "editor") {
    isEditorFullScreen.value = !isEditorFullScreen.value;
    isPreviewFullScreen.value = false;
    emits("update:isEditorFullScreen", isEditorFullScreen.value);
    emits("update:isPreviewFullScreen", false);
  } else if (cardType === "preview") {
    isPreviewFullScreen.value = !isPreviewFullScreen.value;
    isEditorFullScreen.value = false;
    emits("update:isPreviewFullScreen", isPreviewFullScreen.value);
    emits("update:isEditorFullScreen", false);
  }
}

const toggleTheme = () => {
  theme.global.name.value = theme.global.current.value.dark ? "light" : "dark";
};
</script>
