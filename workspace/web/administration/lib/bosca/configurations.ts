import { Api } from './api'
import {
  type ConfigurationFragment,
  type ConfigurationInput,
  DeleteConfigurationDocument,
  GetConfigurationDocument,
  GetConfigurationsDocument,
  SetConfigurationDocument,
} from '~/lib/graphql/graphql'
import { NetworkClient } from '~/lib/bosca/networkclient'
import type { AsyncData } from '#app/composables/asyncData'

export class Configurations<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  getConfigurationsAsyncData(): AsyncData<Array<ConfigurationFragment> | null, any> {
    return this.executeAndTransformAsyncData(
        GetConfigurationsDocument,
        {},
        (data) => {
          if (!data) return null
          return data.configurations?.all as Array<ConfigurationFragment>
        },
    )
  }

  async getConfigurations(): Promise<Array<ConfigurationFragment>> {
    const response = await this.network.execute(GetConfigurationsDocument)
    return response?.configurations.all as Array<ConfigurationFragment>
  }

  async getConfiguration(key: string): Promise<ConfigurationFragment | null> {
    const response = await this.network.execute(GetConfigurationDocument, { key })
    return response?.configurations.configuration as ConfigurationFragment | null
  }

  async setConfiguration(configuration: ConfigurationInput): Promise<ConfigurationFragment> {
    const response = await this.network.execute(SetConfigurationDocument, {
      configuration,
    })
    return response?.configurations?.setConfiguration as ConfigurationFragment
  }

  async deleteConfiguration(key: string): Promise<string> {
    const response = await this.network.execute(DeleteConfigurationDocument, {
      key,
    })
    return response?.configurations?.deleteConfiguration as string
  }
}
