<template>
  <v-app>
    <v-row>
      <v-col cols="6">
        <v-card class="editor-card">
          <v-card-text>
            <textarea
              v-model="markdownText"
              rows="10"
              class="editor-textarea"
            ></textarea>
          </v-card-text>
        </v-card>
      </v-col>

      <v-col cols="6">
        <v-card class="preview-card">
          <v-card-text>
            <div v-html="parsedMarkdown" class="preview-content"></div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-app>
</template>

<script>
import { computed, ref } from "vue";
import { marked } from "marked";

export default {
  setup() {
    const markdownText = ref(`# Milkdown Vue Commonmark

> You're scared of a world where you're needed.

This is a demo for using Milkdown with **Vue**.`);

    const parsedMarkdown = computed(() => {
      return marked(markdownText.value);
    });

    return { markdownText, parsedMarkdown };
  },
};
</script>

<style scoped>
.card {
  margin: 20px;
  padding: 20px;
}
.headline {
  font-size: 24px;
  font-weight: bold;
  margin-bottom: 10px;
}
.editor-card,
.preview-card {
  height: 100vh;
  overflow-y: auto;
  border-radius: "0";
}

.editor-textarea {
  height: 100vh;
  overflow-y: auto;
}
.preview-content {
  height: 100%;
  padding: 16px;
}
</style>
