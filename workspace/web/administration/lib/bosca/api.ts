import type { ExecuteOptions, NetworkClient } from '~/lib/bosca/networkclient'
import type { AsyncData } from '#app/composables/asyncData'
import type { NuxtNetworkClient } from '~/lib/bosca/nuxtnetworkclient'
import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core'

export abstract class Api<T extends NetworkClient> {
  protected constructor(protected network: T) {
  }

  protected getNetworkClient<C extends NetworkClient>(): C {
    return this.network as unknown as C
  }

  protected executeAndTransformAsyncData<T, V, R>(
    document: DocumentNode<T, V>,
    variables: V | any | null = null,
    transform: (data: T | null) => R | null,
    options: ExecuteOptions | undefined = undefined,
  ): AsyncData<R | null, any> {
    const client = this.getNetworkClient<NuxtNetworkClient>()
    const data = client.executeAsyncData<T, V>(document, variables, options)
    return this.transformAsyncData<T, R>(data, transform)
  }

  protected transformAsyncData<T1, T2>(
    data: AsyncData<T1 | null, any>,
    transform: (data: T1 | null) => T2 | null,
  ): AsyncData<T2 | null, any> {
    const transformed = computed(() => {
      return transform(data.data.value)
    })
    return {
      data: transformed,
      refresh: data.refresh,
      execute: data.execute,
      clear: data.clear,
      error: data.error,
      status: data.status,
    } as unknown as AsyncData<T2 | null, any>
  }
}
