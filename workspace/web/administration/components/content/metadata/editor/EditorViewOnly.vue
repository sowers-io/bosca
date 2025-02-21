<script setup lang="ts">
import {newEditor} from "~/lib/editor/editor.ts";
import type {DocumentFragment, MetadataFragment} from "~/lib/graphql/graphql.ts";
import {EditorContent} from "@tiptap/vue-3";
const props = defineProps<{
  document: DocumentFragment,
  metadata: MetadataFragment
}>()
const editor = newEditor(
    props.document,
    props.metadata,
    null,
    null,
    null,
    false
)
</script>
<template>
  <editor-content :editor="editor"/>
</template>