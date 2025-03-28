import { NetworkClient } from '~/lib/bosca/networkclient'
import { Security } from '~/lib/bosca/security'
import { ContentCollections } from '~/lib/bosca/contentcollection'
import { ContentMetadata } from '~/lib/bosca/contentmetadata'
import { Listeners } from '~/lib/bosca/listeners'
import { Workflows } from '~/lib/bosca/workflows'
import { Search } from '~/lib/bosca/search'
import { Profiles } from '~/lib/bosca/profiles'
import { Configurations } from '~/lib/bosca/configurations'
import {
  type CollectionFragment,
  GetSlugDocument,
  type MetadataFragment,
} from '~/lib/graphql/graphql.ts'

export class BoscaClient<T extends NetworkClient> {
  readonly security: Security<T>
  readonly collections: ContentCollections<T>
  readonly metadata: ContentMetadata<T>
  readonly workflows: Workflows<T>
  readonly search: Search<T>
  readonly profiles: Profiles<T>
  readonly listeners: Listeners<T>
  readonly configurations: Configurations<T>

  constructor(private readonly network: T) {
    this.profiles = new Profiles<T>(network)
    this.security = new Security<T>(network)
    this.collections = new ContentCollections<T>(network)
    this.metadata = new ContentMetadata<T>(network)
    this.workflows = new Workflows<T>(network)
    this.search = new Search<T>(network)
    this.listeners = new Listeners<T>(network)
    this.configurations = new Configurations<T>(network)
  }

  async get(
    slug: string,
  ): Promise<MetadataFragment | CollectionFragment | null> {
    const response = await this.network.execute(GetSlugDocument, {
      slug: slug,
    })
    return response?.content?.slug as
      | MetadataFragment
      | CollectionFragment
      | null
  }
}
