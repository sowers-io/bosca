<script setup lang="ts">
import type {
  GuideFragment,
  GuideStep,
  MetadataFragment,
  MetadataRelationshipFragment,
  ParentCollectionFragment,
} from '~/lib/graphql/graphql.ts'

defineProps<{
  metadata: MetadataFragment
  relationships: Array<MetadataRelationshipFragment>
  parents: Array<ParentCollectionFragment>
  guide: GuideFragment
  currentStep: GuideStep | null
}>()

const currentStep = defineModel('currentStep', { type: Object, default: null })
</script>
<template>
  <div class="flex items-center">
    <Button
      variant="ghost"
      :disabled="!currentStep"
      @click="currentStep = null"
    >
      <Icon name="i-lucide-arrow-left" />
    </Button>
    <span class="ms-2">
      {{ currentStep ? 'Modules' : 'Steps' }}
    </span>
  </div>
  <div v-if="!currentStep" class="mt-4">
    <Table v-if="guide.steps && guide.steps.length > 0">
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(step, index) in guide.steps || []"
          :key="step.metadata?.id || index.toString()"
          class="cursor-pointer"
        >
          <TableCell @click="currentStep = step as GuideStep">
            {{ step.metadata?.name || ('Step ' + (index + 1)) }}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
    <div v-else class="p-4">
      No steps found.
    </div>
  </div>
  <div v-else-if="currentStep" class="mt-4">
    <Table v-if="currentStep.modules && currentStep.modules.length > 0">
      <TableHeader>
        <TableRow>
          <TableHead>Name</TableHead>
        </TableRow>
      </TableHeader>
      <TableBody>
        <TableRow
          v-for="(module, index) in currentStep.modules || []"
          :key="module.id"
          class="cursor-pointer"
        >
          <TableCell>
            {{ module.metadata?.name || ('Module ' + (index + 1)) }}
          </TableCell>
        </TableRow>
      </TableBody>
    </Table>
    <div v-else class="p-4">
      No modules found.
    </div>
  </div>
</template>
