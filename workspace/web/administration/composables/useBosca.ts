import { createSharedComposable } from '@vueuse/core'
import { BoscaClient } from '@/lib/bosca/client'
import { NetworkClient } from '~/lib/bosca/networkclient'
import { NuxtNetworkClient } from '~/lib/bosca/nuxtnetworkclient'

let _client: BoscaClient<NuxtNetworkClient> | null = null

function _useBoscaClient(): BoscaClient<NuxtNetworkClient> {
  if (!_client) {
    _client = new BoscaClient<NuxtNetworkClient>(new NuxtNetworkClient())
  }
  return _client
}

export const useBoscaClient = createSharedComposable(_useBoscaClient)
