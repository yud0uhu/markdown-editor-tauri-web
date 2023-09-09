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

<script lang="ts">
import hljs from "highlight.js";
import { reactive, computed, ref } from "vue";
import { marked } from "marked";
import "highlight.js/styles/nord.css";
import { mdiPencil, mdiEye } from "@mdi/js";
export default {
  props: {
    isEditorFullScreen: Boolean,
    isPreviewFullScreen: Boolean,
    toggleTheme: Function,
  },
  data: () => ({
    mdiPencil,
    mdiEye,
  }),
  setup(props) {
    const isEditorFullScreen = ref(false);
    const isPreviewFullScreen = ref(false);
    const markdownText = ref(`# Milkdown Vue Commonmark

> You're scared of a world where you're needed.

This is a demo for using Milkdown with **Vue**.

function init() {
  var name = "Mozilla";
  function displayName() {
    console.log(name);
  }
  displayName();
}
Init();
`);

    const parsedMarkdown = computed(() => {
      return marked(markdownText.value);
    });

    marked.use({
      renderer: {
        code(code, language) {
          language = language ? language : "";
          const validLanguage = hljs.getLanguage(language)
            ? language
            : "plaintext";
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

    return {
      marked,
      markdownText,
      parsedMarkdown,
      editorConfigs,
      isEditorFullScreen,
      isPreviewFullScreen,
      editorStyle,
      previewStyle,
    };
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
