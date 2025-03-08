# Collections

<primary-label ref="bosca"/>
<secondary-label ref="beta"/>

Collections are a fundamental part of Bosca, designed to address diverse use cases. They allow you to:

- Organize content in a folder-like structure.
- Create personalized, user-centric content collections.
- Act as queue-based data structures for curated experiences.
- Build app UIs that showcase your content in a structured, curated format.
- Dynamically control app navigation.

With their versatility, collections empower you to craft tailored experiences, streamline content organization, and enhance app functionality.

## Collection Model

```graphql
union CollectionItem = Collection | Metadata

type Collection {
    attributes(filter: AttributesFilterInput): JSON
    collections(limit: Int!, offset: Int!): [Collection!]!
    created: DateTime!
    description: String
    id: String!
    itemAttributes: JSON
    items(limit: Int!, offset: Int!): [CollectionItem!]!
    labels: [String!]!
    metadata(limit: Int!, offset: Int!): [Metadata!]!
    modified: DateTime!
    name: String!
    ordering: JSON
    parentCollections(limit: Int!, offset: Int!): [Collection!]!
    permissions: [Permission!]!
    public: Boolean!
    publicList: Boolean!
    ready: DateTime
    systemAttributes: JSON
    traitIds: [String!]!
    type: CollectionType!
    workflow: CollectionWorkflow!
}
```

See the full schema: [Full Schema](/graphql/) {{ className: "text-sm" }}