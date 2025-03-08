# Metadata

<primary-label ref="bosca"/>
<secondary-label ref="beta"/>

Metadata is the foundation of content management in Bosca. It can reference internal or external content, serve as a
container for content, all of which also serves for Workflow operations. Whether dealing with structured or
unstructured data, Metadata provides the framework to manage and deliver this content effectively.

## Metadata Model

```graphql
type Metadata {
    attributes(filter: AttributesFilterInput): JSON
    content: MetadataContent!
    created: DateTime!
    id: String!
    itemAttributes: JSON
    labels: [String!]!
    languageTag: String!
    modified: DateTime!
    name: String!
    parentCollections(limit: Int!, offset: Int!): [Collection!]!
    parentId: String
    permissions: [Permission!]!
    public: Boolean!
    publicContent: Boolean!
    publicSupplementary: Boolean!
    ready: DateTime
    relationships: [MetadataRelationship!]!
    source: MetadataSource!
    supplementary(key: String): [MetadataSupplementary!]!
    systemAttributes: JSON
    traitIds: [String!]!
    type: MetadataType!
    uploaded: DateTime
    version: Int!
    workflow: MetadataWorkflow!
}

type MetadataContent {
    json: JSON!
    length: Int
    text: String!
    type: String!
    urls: MetadataContentUrls!
}

type MetadataContentUrls {
    download: SignedUrl!
    upload: SignedUrl!
}

type MetadataRelationship {
    attributes: JSON!
    id: String!
    metadata: Metadata!
    relationship: String!
}

type MetadataSource {
    id: String
    identifier: String
}

type MetadataSupplementary {
    attributes: JSON
    content: MetadataSupplementaryContent!
    created: String!
    key: String!
    metadataId: String!
    modified: String!
    name: String!
    source: MetadataSupplementarySource!
    uploaded: String
}

type MetadataSupplementaryContent {
    json: JSON!
    length: Int
    text: String!
    type: String!
    urls: MetadataSupplementaryContentUrls!
}

type MetadataSupplementaryContentUrls {
    download: SignedUrl!
    upload: SignedUrl!
}

type MetadataSupplementarySource {
    id: String!
    identifier: String
}
```

See the full schema: [Full Schema](/graphql/) {{ className: "text-sm" }}