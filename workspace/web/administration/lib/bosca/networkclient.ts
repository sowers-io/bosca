import type { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core'

export class ClientError extends Error {
  constructor(errors: any[]) {
    super(errors[0].message)
    this.name = 'ClientError'
  }
}

export interface ExecuteOptions {
  authenticate?: boolean | undefined
  post?: boolean | undefined
  query?: string | undefined
  token?: string | undefined
  username?: string | undefined
  password?: string | undefined
  url?: string | undefined
  immediate?: boolean | undefined
  watch?: any[] | undefined
}

export interface NetworkRequest {
  headers: Headers
  query: any
  method: string
  body: any
}

export interface SubscriptionOptions<T> extends ExecuteOptions {
  onData: (data: T) => void
  onError?: (error: ClientError) => void
}

export class NetworkClient {
  private _token: string | null = null

  get token(): string | null {
    return this._token
  }

  set token(value: string | null | undefined) {
    this._token = value || null
  }

  protected newQuery<T, V>(
    document: DocumentNode<T, V>,
    variables: any | null,
  ): { [key: string]: any } {
    // @ts-ignore: any
    const meta = document['__meta__']
    const extensions = {
      persistedQuery: {
        version: 1,
        sha256Hash: meta!['hash'],
      },
    }
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    const queryVariables: { [key: string]: any } = {}
    if (variables) {
      for (const variable in variables) {
        const v = variables[variable]
        if (typeof isRef !== 'undefined' && isRef(v)) {
          queryVariables[variable] = unref(v)
        } else {
          queryVariables[variable] = variables[variable]
        }
      }
    }
    return {
      extensions,
      variables: queryVariables,
    }
  }

  subscribe<T, V>(
    document: DocumentNode<T, V>,
    variables: any | null = null,
    options: SubscriptionOptions<T>,
  ) {
    throw new Error('Method not implemented.')
  }

  protected async fetch(url: string, request: NetworkRequest): Promise<any> {
    if (request.query) {
      url = url + '?extensions=' +
        encodeURIComponent(JSON.stringify(request.query.extensions)) +
        '&variables=' +
        encodeURIComponent(JSON.stringify(request.query.variables))
    }
    const response = await fetch(url, request)
    return await response.json()
  }

  async execute<T, V>(
    document: DocumentNode<T, V>,
    variables: any | null = null,
    options: ExecuteOptions | undefined = undefined,
  ): Promise<T | null> {
    const headers = new Headers()
    headers.set('Content-Type', 'application/json')
    headers.set('Accept', 'application/json')
    const op = document.definitions.find((d) =>
      d.kind === 'OperationDefinition'
    )
    const mutation = op?.operation === 'mutation'
    if (options && options.username && options.password) {
      headers.set(
        'Authorization',
        'Basic ' + btoa(options.username + ':' + options.password),
      )
    } else if (options && options.token) {
      headers.set('Authorization', 'Bearer ' + options.token)
    } else if (options?.authenticate !== false) {
      const token = this.token
      if (token) {
        headers.set('Authorization', 'Bearer ' + token)
      }
    }
    const url = options && options.url
      ? options.url
      : import.meta.client
      ? useRuntimeConfig().public.graphqlUrl
      : useRuntimeConfig().graphqlUrl
    let body = undefined
    if (options && options.query) {
      body = JSON.stringify({
        query: options.query,
        variables: variables,
      })
    } else if (mutation) {
      body = JSON.stringify(this.newQuery(document, variables))
    }
    const request = {
      headers: headers,
      query: !mutation && !(options && options.post)
        ? this.newQuery(document, variables)
        : undefined,
      method: mutation || (options && options.post) ? 'POST' : 'GET',
      body: body,
    }
    const response = await this.fetch(url.toString(), request)
    if (response) {
      const r = response as any
      if (r.errors) {
        throw new ClientError(r.errors)
      }
      return r.data as T
    }
    return null
  }
}
