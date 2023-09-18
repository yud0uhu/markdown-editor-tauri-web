<template>
  <v-app>
    <v-row>
      <v-col :style="editorStyle">
        <v-card class="editor-card width-100">
          <v-card-text>
            <textarea
              v-if="!isPreviewFullScreen"
              v-model="markdownText"
              :editorConfigs="editorConfigs"
              rows="10"
              class="editor-textarea"
            ></textarea>
          </v-card-text>
        </v-card>
      </v-col>
      <v-col :style="previewStyle">
        <v-card class="preview-card width-100">
          <v-card-text>
            <div
              v-if="!isEditorFullScreen"
              rows="10"
              v-html="parsedMarkdown"
              class="preview-content"
            ></div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>
  </v-app>
</template>

<script setup lang="ts">
import hljs from "highlight.js";
import { reactive, computed, ref, defineProps, watch } from "vue";
import { marked } from "marked";
import "highlight.js/styles/nord.css";

const emits = defineEmits(["update:onMarkdownUpdate", "update:markdownText"]);

const props = defineProps({
  isEditorFullScreen: Boolean,
  isPreviewFullScreen: Boolean,
  toggleTheme: Function,
  markdownText: String,
  onMarkdownUpdate: Function,
});
const isEditorFullScreen = ref(false);
const isPreviewFullScreen = ref(false);
const markdownText = ref("");

const onMarkdownUpdate = (value: string) => {
  emits("update:onMarkdownUpdate", value);
  emits("update:markdownText", value);
};
watch(
  () => props.markdownText,
  (newValue) => {
    if (newValue !== markdownText.value && newValue) {
      markdownText.value = newValue;
      console.log("watch:" + markdownText.value);
      onMarkdownUpdate(markdownText.value);
    }
  }
);

const parsedMarkdown = computed(() => {
  emits("update:onMarkdownUpdate", markdownText.value);
  return marked(markdownText.value);
});

marked.use({
  renderer: {
    code(code, language) {
      language = language ? language : "";
      const validLanguage = hljs.getLanguage(language) ? language : "plaintext";
      return `<pre><code class="hljs ${validLanguage}">${
        hljs.highlight(validLanguage, code).value
      }</code></pre>`;
    },
  },
});

const editorConfigs = reactive({
  codeBlock: {
    languages: [
      { language: "javascript", label: "JS" },
      { language: "css", label: "CSS" },
      { language: "html", label: "HTML" },
    ],
  },
});

const editorStyle = computed(() => ({
  display: props.isPreviewFullScreen ? "none" : "block",
}));

const previewStyle = computed(() => ({
  display: props.isEditorFullScreen ? "none" : "block",
}));
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
  width: 100%;
  height: 100vh;
  overflow-y: auto;
}
.preview-content {
  height: 100%;
  padding: 16px;
}

.v-btn {
  margin-right: 8px;
}

.header {
  margin-bottom: 16px;
}

.editor-fullscreen {
  width: 100%;
}

.preview-card {
  display: flex;
}

.preview-col {
  transition: width 0.3s;
}

.v-tooltip {
  display: inline-block;
  margin: 0 8px;
}

.width-100 {
  width: 100%;
}
</style>
