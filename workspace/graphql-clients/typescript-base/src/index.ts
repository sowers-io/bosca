export interface Data {
  extensions: any,
  variables: any
}

export interface RequestParams {
  url: string,
  // eslint-disable-next-line no-undef
  init: RequestInit
}

export class ClientError extends Error {
  public code: number
  public errors: any

  constructor(code: number, errors: any) {
    super('Request Error')
    this.code = code
    this.errors = errors
  }
}

export class RequestInterceptor {
  async onRequest(params: RequestParams): Promise<RequestParams> {
    return params
  }
  async onResponse<T>(response: Response): Promise<T> {
    if (!response.ok) {
      throw new ClientError(response.status, response.statusText)
    }
    const res = await response.json()
    if (res.errors) {
      throw new ClientError(response.status, res.errors)
    }
    return res.data as T
  }
}

export class BaseClient {

  private readonly url: string
  public interceptor: RequestInterceptor = new RequestInterceptor()

  public constructor(url: string) {
    this.url = url
  }

  public async execute<T>(data: Data, mutation: boolean): Promise<T> {
    const headers = new Headers()
    headers.set('Content-Type', 'application/json')
    headers.set('Accept', 'application/json')
    let url = this.url
    if (!mutation) {
      url += '?extensions=' + encodeURIComponent(JSON.stringify(data.extensions))
      if (data.variables) {
        url += '&variables=' + encodeURIComponent(JSON.stringify(data.variables))
      }
    }
    const params = await this.interceptor.onRequest({
      url,
      init: {
        headers,
        method: mutation ? 'POST' : 'GET',
        body: mutation ? JSON.stringify(data) : undefined,
      },
    })
    const response = await fetch(params.url, params.init)
    return await this.interceptor.onResponse<T>(response)
  }
}