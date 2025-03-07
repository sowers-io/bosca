import type { NetworkClient } from '~/lib/bosca/networkclient'
import { Api } from '~/lib/bosca/api'
import {
  GetCurrentProfileDocument,
  GetProfileDocument,
  GetProfilesDocument,
  type Profile,
  type ProfileFragment,
  ProfileVisibility,
} from '~/lib/graphql/graphql'
import type { AsyncData } from '#app/composables/asyncData'

export class Profiles<T extends NetworkClient> extends Api<T> {
  private profile: Profile | null = null

  constructor(network: T) {
    super(network)
  }

  getProfiles(
    offset: number | Ref<number, null>,
    limit: number | Ref<number, null>,
  ): AsyncData<Array<ProfileFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetProfilesDocument,
      { offset, limit },
      (data) => {
        if (!data) return null
        return data.profiles?.all as Array<ProfileFragment>
      },
    )
  }

  async getProfile(id: string): Promise<Profile> {
    return (await this.network.execute(GetProfileDocument, { id }))?.profiles
      ?.profile as Profile
  }

  async getCurrentProfile(): Promise<Profile> {
    if (this.profile) return this.profile
    this.profile =
      ((await this.network.execute(GetCurrentProfileDocument))?.profiles
        ?.current || {
        name: 'Anonymous',
        attributes: [],
        visibility: ProfileVisibility.User,
      }) as Profile
    return this.profile
  }
}
