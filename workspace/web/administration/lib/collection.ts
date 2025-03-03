import type {
    CollectionFragment,
    CollectionInput, OrderingInput
} from '~/lib/graphql/graphql.ts'

export function toCollectionInput(collection: CollectionFragment): CollectionInput {
    const input: CollectionInput = {
        attributes: collection.attributes,
        categoryIds: collection.categories?.map((c) => c.id) || [],
        collectionType: collection.collectionType,
        description: collection.description,
        labels: collection.labels,
        name: collection.name,
        ordering: collection.ordering as Array<OrderingInput>,
        slug: collection.slug,
        templateMetadataId: collection.templateMetadata?.id,
        templateMetadataVersion: collection.templateMetadata?.version,
        traitIds: collection.traitIds,
    }
    return input
}
