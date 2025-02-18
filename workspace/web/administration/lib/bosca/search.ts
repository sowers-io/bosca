import type { NetworkClient } from '~/lib/bosca/networkclient'
import { Api } from '~/lib/bosca/api'
import {
  type CollectionIdNameFragment,
  ExecuteSearchDocument,
  type MetadataIdNameFragment,
  ProfileIdNameFragment,
} from '~/lib/graphql/graphql'
import type { AsyncData } from '#app/composables/asyncData'

export class Search<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  searchAsyncData(
    query: string | Ref<string, string>,
    filter: string | Ref<string | null, string | null> | null,
    offset: number | Ref<number, null>,
    limit: number | Ref<number, null>,
    storageSystemId: string | Ref<string, string>,
  ): AsyncData<
    Array<
      CollectionIdNameFragment | MetadataIdNameFragment | ProfileIdNameFragment
    > | null,
    any
  > {
    return this.executeAndTransformAsyncData(
      ExecuteSearchDocument,
      {
        query,
        filter,
        offset,
        limit,
        storageSystemId,
      },
      (data) => {
        if (!data) return null
        const results = data?.search?.documents.map((d) =>
          d.collection || d.metadata || d.profile!
        )
        return results
      },
    )
  }
}
