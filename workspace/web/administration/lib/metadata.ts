import type {
    MetadataFragment,
    MetadataInput, MetadataProfileInput,
    MetadataSourceInput,
} from '~/lib/graphql/graphql.ts'

export function toMetadataInput(metadata: MetadataFragment): MetadataInput {
    const profiles = []
    for (const profile of metadata.profiles) {
        profiles.push({
            profileId: profile.profile!.id,
            relationship: profile.relationship,
        } as MetadataProfileInput)
    }
    const categoryIds = []
    for (const category of metadata.categories) {
        categoryIds.push(category.id)
    }
    const input = {
        attributes: metadata.attributes,
        categoryIds: categoryIds,
        contentLength: metadata.content.length,
        contentType: metadata.content.type,
        labels: metadata.labels,
        languageTag: metadata.languageTag,
        metadataType: metadata.type,
        name: metadata.name,
        parentId: metadata.parentId,
        profiles: profiles,
        source: metadata.source && (metadata.source.id || metadata.source.identifier)
            ? metadata.source as MetadataSourceInput
            : null,
        traitIds: metadata.traitIds,
        version: metadata.version,
    }
    console.log(input)
    return input
}
