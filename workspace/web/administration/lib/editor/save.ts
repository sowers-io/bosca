import type {
  CollectionIdNameFragment,
  DocumentFragment,
  DocumentInput,
  DocumentTemplateFragment,
  MetadataFragment,
  MetadataRelationshipFragment, ParentCollectionFragment,
} from '~/lib/graphql/graphql.ts'
import { AttributeUiType } from '~/lib/graphql/graphql.ts'
import type { BoscaClient } from '~/lib/bosca/client.ts'
import type { Reactive } from 'vue'
import { toMetadataInput } from '~/lib/metadata.ts'
import slugify from "slugify";

export async function save(
  client: BoscaClient<any>,
  document: DocumentFragment,
  metadata: MetadataFragment,
  template: DocumentTemplateFragment | null,
  parents: ParentCollectionFragment[],
  title: string,
  relationships: MetadataRelationshipFragment[],
  attributes: Reactive<Map<string, AttributeState>>,
  content: any,
) {
  const newDocument: DocumentInput = {
    templateMetadataId: document.template?.id,
    templateMetadataVersion: document.template?.version,
    title: title,
    content: {
      document: content,
    },
  }
  const input = toMetadataInput(toRaw(metadata))
  input.document = newDocument
  input.name = title
  input.slug = slugify(input.name).toLocaleLowerCase()

  for (const attribute of template?.attributes || []) {
    const attr = toRaw(attributes.get(attribute.key))
    if (attr) {
      switch (attr.ui) {
        case AttributeUiType.Textarea:
        case AttributeUiType.Input:
          if (!input.attributes) input.attributes = {}
          input.attributes[attr.key] = attr.value
          break
        case AttributeUiType.Profile:
          if (!input.profiles) input.profiles = []
          input.profiles = (input.profiles || []).filter((p) =>
            p.relationship !== attr.configuration.relationship
          )
          if (attr.value) {
            input.profiles.push(attr.value)
          }
          break
      }
    }
  }

  await client.metadata.edit(metadata.id, input)

  for (const collection of parents || []) {
    await client.collections.removeMetadata(collection.id, metadata.id)
  }

  for (const attribute of template?.attributes || []) {
    const attr = attributes.get(attribute.key)
    if (attr) {
      switch (attr.ui) {
        case AttributeUiType.Collection: {
          if (attr.list) {
            const collections = attr.value
            if (!collections) continue
            for (const collection of collections) {
              await client.collections.addMetadata(collection.id, metadata.id)
            }
          } else if (attr.value) {
            const collection = attr.value as CollectionIdNameFragment
            await client.collections.addMetadata(collection.id, metadata.id)
          }
          break
        }
        case AttributeUiType.Image:
        case AttributeUiType.File:
        case AttributeUiType.Metadata: {
          const removeRelationshipId = relationships.find((r) =>
            r.relationship === attr.configuration.relationship
          )?.metadata?.id
          if (removeRelationshipId) {
            await client.metadata.removeRelationship(
              metadata.id,
              removeRelationshipId,
              attr.configuration.relationship,
            )
          }
          if (!attr.value) continue
          const relationship = toRaw(attr.value)
          await client.metadata.addRelationship({
            id1: metadata.id,
            id2: relationship.metadata.id,
            attributes: {},
            relationship: attr.configuration.relationship,
          })
          break
        }
      }
    }
  }
}
