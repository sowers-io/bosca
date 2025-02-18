import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core'
import { useWebSocket } from '@vueuse/core'
import type { AsyncData } from '#app/composables/asyncData'
import { useAsyncData } from '#app'
import type { CookieRef } from '#app/composables/cookie'
import {
  ClientError,
  type ExecuteOptions,
  NetworkClient,
  type NetworkRequest,
  type SubscriptionOptions,
} from '~/lib/bosca/networkclient'

export class NuxtNetworkClient extends NetworkClient {
  private _cookie: CookieRef<string> | null = null

  override get token(): string | null {
    if (!this._cookie) {
      this._cookie = useCookie('_bat')
    }
    return this._cookie.value || null
  }

  override set token(value: string | null | undefined) {
    if (!this._cookie) {
      this._cookie = useCookie('_bat')
    }
    this._cookie.value = value || ''
  }

  protected override async fetch(
    url: string,
    request: NetworkRequest,
  ): Promise<any> {
    return await $fetch(url, request as unknown as any)
  }

  override subscribe<T, V>(
    document: DocumentNode<T, V>,
    variables: any | null = null,
    options: SubscriptionOptions<T>,
  ) {
    if (import.meta.server) return
    const url = useRuntimeConfig().public.graphqlWsUrl
    let ids = new Date().getTime()
    // deno-lint-ignore no-this-alias
    const self = this
    const { send } = useWebSocket(url, {
      autoReconnect: true,
      autoClose: true,
      autoConnect: true,
      onConnected: () => {
        send(JSON.stringify({
          type: 'connection_init',
          payload: { token: self.token },
        }))
      },
      onMessage: (socket, event) => {
        const body = JSON.parse(event.data)
        switch (body.type) {
          case 'connection_ack':
            send(JSON.stringify({
              id: (ids++).toString(),
              type: 'subscribe',
              payload: this.newQuery(document, variables),
            }))
            break
          case 'data':
            if (body.errors) {
              if (options.onError) {
                options.onError(new ClientError(body.errors))
              }
            } else {
              options.onData(body.payload.data as T)
            }
            break
          case 'complete':
            console.error('!!!!!complete!!!!!', body, event)
            break
          default:
            console.error('!!!!!unhandled!!!!', body, event)
            break
        }
      },
      onError: (socket, event) => {
        console.error('!!!!!errror!!!!!!', event)
      },
      protocols: [
        'graphql-ws',
      ],
    })
  }

  executeAsyncData<T, V>(
    document: DocumentNode<T, V>,
    variables: V | any | null = null,
    options: ExecuteOptions | undefined = undefined,
  ): AsyncData<T | null, any> {
    const refs = []
    if (variables) {
      for (const variable in variables) {
        const v = variables[variable]
        if (isRef(v)) {
          refs.push(v)
        }
      }
    }
    const key = '/graphql' + JSON.stringify(this.newQuery(document, variables))
    // deno-lint-ignore no-this-alias
    const self = this
    return useAsyncData(key, async (_) => {
      return await self.execute(document, variables, options)
    }, {
      watch: refs,
      deep: false,
      immediate: options?.immediate,
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
    }) as AsyncData<T | null, any>
  }
}
