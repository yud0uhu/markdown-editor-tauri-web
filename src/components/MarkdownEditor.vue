<template>
  <v-app>
    <v-container fluid>
      <v-row>
        <v-col :style="editorStyle" class="bg-grey-darken-4">
          <ToolBar @insert="insertMarkdownText" />
          <v-card class="editor-card width-100 bg-grey-darken-4">
            <v-card-text>
              <textarea
                @input="updateSuggestList(markdownText)"
                autocomplete="on"
                list="food"
                v-if="!isPreviewFullScreen"
                v-model="markdownText"
                :editorConfigs="editorConfigs"
                rows="10"
                class="editor-textarea bg-grey-darken-4"
              ></textarea>

              <v-menu
                bottom
                transition="scale-transition"
                v-model="showSuggest"
                focusable
              >
                <v-list>
                  <v-list-item
                    autofocus
                    v-for="(item, i) in suggestList"
                    :key="i"
                    :value="item"
                    color="primary"
                    @click="selectSuggestion(item.text)"
                  >
                    <template v-slot:prepend>
                      <v-icon :icon="item.icon" v-if="item.icon"></v-icon>
                    </template>

                    <v-list-item-title v-text="item.text"></v-list-item-title>
                  </v-list-item>
                </v-list>
              </v-menu>
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
    </v-container>
  </v-app>
</template>

<script setup lang="ts">
import hljs from "highlight.js";
import { reactive, computed, ref, defineProps, watch } from "vue";
import MarkdownIt from "markdown-it";
import "highlight.js/styles/nord.css";
import ToolBar from "./ToolBar.vue";
import emoji from "markdown-it-emoji";

const md = new MarkdownIt();
const emits = defineEmits(["update:onMarkdownUpdate", "update:markdownText"]);
md.use(emoji);
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

const insertMarkdownText = (content: string) => {
  markdownText.value += content;
  console.log("Markdownが更新されました:", content);
  emits("update:markdownText", content);
};
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
  return md.render(markdownText.value);
});

md.options.highlight = function (code, lang) {
  lang = lang || "plaintext";
  const validLanguage = hljs.getLanguage(lang) ? lang : "plaintext";
  return `<pre><code class="hljs ${validLanguage}">${
    hljs.highlight(validLanguage, code).value
  }</code></pre>`;
};

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
const inputText = ref("");
const suggestList = ref([
  {
    text: "",
    icon: "",
  },
]);
const showSuggest = ref(false);
const updateSuggestList = (markdownText: string) => {
  const input = inputText.value.toLowerCase();
  console.log(markdownText);
  // TODO:hサジェスト
  suggestList.value = [];
  if (markdownText.trim().startsWith("#")) {
    suggestList.value = [
      { text: "# h1", icon: "" },
      { text: "## h2", icon: "" },
      { text: "### h3", icon: "" },
      { text: "#### h4", icon: "" },
      { text: "##### h5", icon: "" },
      { text: "###### h6", icon: "" },
    ];
  }

  if (isEmoji) {
    suggestList.value = [
      { text: ":smile:", icon: "mdi-emoticon-happy" },
      { text: ":heart:", icon: "mdi-heart" },
      { text: ":star:", icon: "mdi-star" },
      { text: ":+1:", icon: "mdi-thumb-up" },
      { text: ":-1:", icon: "mdi-thumb-down" },
      { text: ":bell:", icon: "mdi-bell" },
      { text: ":fire:", icon: "mdi-fire" },
      { text: ":ok_hand:", icon: "mdi-thumb-up" },
      { text: ":rocket:", icon: "mdi-rocket" },
      { text: ":speech_balloon:", icon: "mdi-message-text-outline" },
      { text: ":mag:", icon: "mdi-magnify" },
      { text: ":bulb:", icon: "mdi-lightbulb" },
      { text: ":moneybag:", icon: "mdi-currency-usd" },
      { text: ":camera:", icon: "mdi-camera" },
      { text: ":trophy:", icon: "mdi-trophy" },
      { text: ":gift:", icon: "mdi-gift" },
      { text: ":hourglass:", icon: "mdi-hourglass" },
      { text: ":envelope:", icon: "mdi-email" },
      { text: ":link:", icon: "mdi-link" },
      { text: ":lock:", icon: "mdi-lock" },
    ];
  }

  suggestList.value = suggestList.value.filter((item) =>
    item.text.toLowerCase().includes(input)
  );
  showSuggest.value = !!suggestList.value.length;
};

const selectSuggestion = (suggestion: string) => {
  const newText = markdownText.value + suggestion.substring(1);
  markdownText.value = newText;
  inputText.value = "";
  showSuggest.value = false;
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
