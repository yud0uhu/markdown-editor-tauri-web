<template>
  <v-row class="header" align="end">
    <v-col class="ml-auto">
      <v-toolbar>
        <v-btn @click="toggleFullScreen('editor')" icon v-on="on">
          <v-icon :color="isEditorFullScreen ? 'grey' : ''">mdi-pencil</v-icon>
        </v-btn>
        <v-btn @click="toggleFullScreen('preview')" icon v-on="on">
          <v-icon :color="isPreviewFullScreen ? 'grey' : ''">mdi-eye</v-icon>
        </v-btn>
        <v-btn @click="toggleTheme" icon v-on="on">
          <v-icon>
            {{
              theme.global.current.value.dark
                ? "mdi-lightbulb"
                : "mdi-lightbulb-outline"
            }}
          </v-icon>
        </v-btn>
        <v-spacer></v-spacer>
        <v-btn @click="showDrawer = !showDrawer" icon>
          <v-icon>mdi-menu</v-icon>
        </v-btn>
        <v-navigation-drawer v-model="showDrawer" right>
          <v-list>
            <v-list-item @click="openFile">
              <v-list-item-icon>
                <v-icon>mdi-folder-open</v-icon>
              </v-list-item-icon>
              <v-list-item-content>
                <v-list-item-title>File Open</v-list-item-title>
              </v-list-item-content>
            </v-list-item>

            <v-list-item @click="saveFile">
              <v-list-item-icon>
                <v-icon>mdi-content-save</v-icon>
              </v-list-item-icon>
              <v-list-item-content>
                <v-list-item-title>File Save</v-list-item-title>
              </v-list-item-content>
            </v-list-item>
          </v-list>
        </v-navigation-drawer>
      </v-toolbar>
    </v-col>
  </v-row>
</template>

<script setup lang="ts">
import { useTheme } from "vuetify/lib/framework.mjs";
import { ref } from "vue";

const props = defineProps({
  isEditorFullScreen: Boolean,
  isPreviewFullScreen: Boolean,
});
const emits = defineEmits([
  "update:isEditorFullScreen",
  "update:isPreviewFullScreen",
]);

const showDrawer = ref(false);
const theme = useTheme();

const isEditorFullScreen = ref(props.isEditorFullScreen);
const isPreviewFullScreen = ref(props.isPreviewFullScreen);
const openFile = () => {
  // TODO: ファイルを開く処理
};

const saveFile = () => {
  // TODO: ファイルを保存する処理
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
