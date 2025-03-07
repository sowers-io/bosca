import { Api } from './api'
import {
  GetGroupsDocument,
  GetPrincipalsDocument,
  type GroupFragment,
  LoginDocument,
  type LoginResponse,
  type PrincipalFragment,
  type ProfileInput,
  SignUpDocument,
  VerifySignUpDocument,
} from '~/lib/graphql/graphql'
import { NetworkClient } from '~/lib/bosca/networkclient'
import type { AsyncData } from '#app/composables/asyncData'

export class Security<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  getGroups(
    offset: number | Ref<number, null>,
    limit: number | Ref<number, null>,
  ): AsyncData<Array<GroupFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetGroupsDocument,
      { offset, limit },
      (data) => {
        if (!data) return null
        return data.security?.groups?.all as Array<GroupFragment>
      },
    )
  }

  getPrincipals(
    offset: number | Ref<number, null>,
    limit: number | Ref<number, null>,
  ): AsyncData<Array<PrincipalFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetPrincipalsDocument,
      { offset, limit },
      (data) => {
        if (!data) return null
        return data.security?.principals?.all as Array<PrincipalFragment>
      },
    )
  }

  async loginWithPassword(
    identifier: string,
    password: string,
  ): Promise<LoginResponse> {
    const response = await this.network.execute(LoginDocument, {
      identifier,
      password,
    })
    if (!response) throw new Error('invalid response')
    // @ts-ignore: not sure why this is confused
    const login = response.security.login.password
    this.network.token = login.token.token
    return login
  }

  async verify(token: string): Promise<void> {
    await this.network.execute(VerifySignUpDocument, {
      token,
    })
  }

  async signUpWithPassword(
    profile: ProfileInput,
    identifier: string,
    password: string,
  ) {
    await this.network.execute(SignUpDocument, {
      profile,
      identifier,
      password,
    })
  }
}

// suspend fun login(identifier: String, password: String) {
//     val response = network.client.query(LoginQuery(identifier, password)).execute()
//     response.validate()
//     network.token = response.data?.security?.login?.password?.token?.token
// }
//
// suspend fun getGroups(): List<Group> {
//     val response = network.client.query(GetGroupsQuery()).execute()
//     response.validate()
//     return response.data?.security?.groups?.all?.map { it.group } ?: emptyList()
// }
//
// suspend fun getPermissionActions(): List<String> {
//     val response = network.client.query(GetPermissionActionsQuery()).execute()
//     response.validate()
//     return response.data?.security?.actions?.map { it.uppercase() } ?: emptyList()
// }
// }
