<script setup lang="ts">
import type {
  ConfigurationFragment,
  PermissionInput,
} from '~/lib/graphql/graphql'
import JsonEditorVue from 'json-editor-vue'
import 'vanilla-jsoneditor/themes/jse-theme-dark.css'

class Configuration {
  readonly configuration: ConfigurationFragment
  value: any

  constructor(configuration: ConfigurationFragment) {
    this.configuration = configuration
    this.value = configuration.value
  }
}

const bosca = useBoscaClient()
const configurations = (await bosca.configurations.getConfigurations()).map(
  (c) => reactive(new Configuration(c)),
)

async function saveConfigurations() {
  for (const config of configurations) {
    await bosca.configurations.setConfiguration({
      description: config.configuration.description,
      key: config.configuration.key,
      permissions: config.configuration.permissions.map((p) => {
        return {
          action: p.action,
          entityId: config.configuration.id,
          groupId: p.group.id,
        } as PermissionInput
      }),
      value: typeof config.value === 'string'
        ? JSON.parse(toRaw(config.value))
        : toRaw(config.value),
    })
    console.log(toRaw(config.value))
  }
}
</script>

<template>
  <div class="space-y-4">
    <h4 class="text-lg font-medium">
      Configurations
    </h4>
    <div
      v-for="(config, index) in configurations"
      :key="config.configuration.id"
      class="border rounded p-4"
    >
      <div>
        <h5 class="font-medium">{{ config.configuration.description }}</h5>
        <p class="text-sm text-gray-500 mb-4">{{ config.configuration.key }}</p>
        <JsonEditorVue
          :mode="'text'"
          :statusBar="false"
          :mainMenuBar="false"
          class="jse-theme-dark rounded-md overflow-hidden"
          v-model="config.value"
        />
      </div>
    </div>
    <Button @click="saveConfigurations">Save</Button>
  </div>
</template>
