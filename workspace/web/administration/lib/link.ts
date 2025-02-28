import type {
    CollectionFragment,
    CollectionIdNameFragment,
    MetadataFragment, MetadataIdNameFragment,
    ProfileFragment, ProfileIdNameFragment
} from "~/lib/graphql/graphql.ts";

export function getLink(item: CollectionFragment | CollectionIdNameFragment | MetadataFragment | MetadataIdNameFragment | ProfileFragment | ProfileIdNameFragment): string {
    switch (item.__typename) {
        case 'Collection':
            return '/collections/' + item.id
        case 'Metadata':
            if (item.content.type === 'bosca/v-document') {
                return '/content/' + item.id
            }
            return '/metadata/edit/' + item.id
        case 'Profile':
            return '/profiles/' + item.id
    }
}