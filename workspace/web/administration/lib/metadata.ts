import type {
  MetadataFragment,
  MetadataInput,
  MetadataSourceInput,
} from '~/lib/graphql/graphql.ts'

export function toMetadataInput(metadata: MetadataFragment): MetadataInput {
  return {
    attributes: metadata.attributes,
    categoryIds: metadata.categories.map((c) => c.id),
    contentLength: metadata.content.length,
    contentType: metadata.content.type,
    labels: metadata.labels,
    languageTag: metadata.languageTag,
    metadataType: metadata.type,
    name: metadata.name,
    parentId: metadata.parentId,
    source:
      metadata.source && (metadata.source.id || metadata.source.identifier)
        ? metadata.source as MetadataSourceInput
        : null,
    traitIds: metadata.traitIds,
    version: metadata.version,
  }
}
