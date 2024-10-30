export enum ActionState {
  DEFAULT,
  ADD_METADATA_TO_COLLECTION,
  ADD_COLLECTION_TO_COLLECTION,
  ADD_METADATA_RELATIONSHIP,
}

export interface ActionHandler {
  setAddMetadataToCollection(): void
  setAddMetadataRelationship(): void
  setAddCollectionToCollection(): void
  openCollection(id: string): void
  createNewMetadata(parent: string): void
  createNewCollection(parent: string): void
}

export interface CommandMenuAction {
  name: string
  link: string
}

export interface Collection {
  id: string
  name: string
  parentCollections: Collection[]
}

export interface Metadata {
  id: string
  name: string
  parentCollections: Collection[]
}

export interface SearchDocument {
  collection: Collection | undefined
  metadata: Metadata | undefined
}

export interface SearchResults {
  documents: SearchDocument[]
}

export interface CurrentItem {
  collection: boolean
  id: string
  item: Collection | Metadata
}
