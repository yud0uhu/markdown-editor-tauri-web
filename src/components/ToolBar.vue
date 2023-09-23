<template>
  <v-toolbar class="bg-black">
    <v-btn @click="InsertBold()">
      <b>B</b>
    </v-btn>
    <v-btn @click="InsertItalic()">
      <i>I</i>
    </v-btn>
    <v-btn @click="InsertStrikethrough()">
      <s>abc</s>
    </v-btn>
    <v-btn @click="InsertHeading()">
      <h>H</h>
    </v-btn>
    <v-spacer></v-spacer>
    <v-btn @click="InsertCheckList()">
      <v-icon>mdi-check-box-outline</v-icon>
    </v-btn>
    <v-btn @click="InsertLink()">
      <v-icon>mdi-link</v-icon>
    </v-btn>
    <v-btn @click="InsertImage()">
      <v-icon>mdi-image</v-icon>
    </v-btn>
    <v-btn @click="InsertTable()">
      <v-icon>mdi-table</v-icon>
    </v-btn>
  </v-toolbar>
</template>

<script setup lang="ts">
import { ref, defineEmits } from "vue";

const emits = defineEmits(["insert"]);

const InsertTable = () => {
  emitInsert(
    "\n" +
      "| Column 1 | Column 2 | Column 3 |\n" +
      "| -------- | -------- | -------- |\n" +
      "| Text     | Text     | Text     |\n"
  );
};
const InsertImage = () => {
  emitInsert("![alt](https://)");
};
const InsertLink = () => {
  emitInsert("[](https://)");
};
const InsertCheckList = () => {
  emitInsert("- [ ] ");
};
const InsertBold = () => {
  emitInsert("****");
};
const InsertItalic = () => {
  emitInsert("**");
};
const InsertStrikethrough = () => {
  emitInsert("~~~~");
};
const InsertHeading = () => {
  emitInsert("#");
};

const emitInsert = (content: string) => {
  if (typeof content === "string") {
    console.log(content);
    emits("insert", content);
  }
};
</script>
<style scoped>
.toolbar {
  flex-wrap: nowrap;
}
.toolbar button {
  margin: 0 4px;
  min-width: 36px;
  min-height: 36px;
  padding: 0;
}
.toolbar button i,
.toolbar button b,
.toolbar button s {
  font-size: 1.5em;
}
</style>
