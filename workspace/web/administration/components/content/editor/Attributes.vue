<script lang="ts" setup>
import {AttributeUiType, type ParentCollectionFragment} from "~/lib/graphql/graphql.ts";
import type {AttributeState} from "~/lib/attribute.ts";
import type {Reactive} from "vue";

defineProps<{
  parents: ParentCollectionFragment[] | null
  attributes: Reactive<Map<string, AttributeState>>
  uploader: Uploader
  editable: boolean
  workflowsEnabled: boolean
  onRunWorkflow: (attribute: AttributeState) => void
}>()
</script>

<template>
  <div v-for="attr in attributes.values()" :key="attr.key" class="mb-4">
    <template v-if="attr.ui === AttributeUiType.Collection && attr.list">
      <ContentEditorCollectionsAttribute
          :collections="parents || []"
          :attribute="attr as AttributeState"
          :editable="editable"
          :workflows-enabled="workflowsEnabled"
          :on-run-workflow="onRunWorkflow"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.Collection && !attr.list">
      <ContentEditorCollectionAttribute
          :collections="parents || []"
          :attribute="attr as AttributeState"
          :editable="editable"
          :workflows-enabled="workflowsEnabled"
          :on-run-workflow="onRunWorkflow"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.Input">
      <ContentEditorInputAttribute
          :attribute="attr as AttributeState"
          :editable="editable"
          :workflows-enabled="workflowsEnabled"
          :on-run-workflow="onRunWorkflow"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.Textarea">
      <ContentEditorTextAreaAttribute
          :attribute="attr as AttributeState"
          :editable="editable"
          :workflows-enabled="workflowsEnabled"
          :on-run-workflow="onRunWorkflow"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.Image">
      <ContentEditorImageAttribute
          :attribute="attr as AttributeState"
          :editable="editable"
          :uploader="uploader"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.File || attr.ui === AttributeUiType.Metadata">
      <ContentEditorFileAttribute
          :attribute="attr as AttributeState"
          :editable="editable"
          :uploader="uploader"
      />
    </template>
    <template v-if="attr.ui === AttributeUiType.Profile">
      <ContentEditorProfileAttribute
          :attribute="attr as AttributeState"
          :editable="editable"
      />
    </template>
  </div>
</template>