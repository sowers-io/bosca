import {
  AttributeUiType,
  type DocumentTemplateAttribute,
} from '~/lib/graphql/graphql.ts'
import { toast } from '~/components/ui/toast'
import type { BoscaClient } from '~/lib/bosca/client.ts'

export class AttributeState {
  readonly key: string
  readonly ui: AttributeUiType
  readonly list: boolean
  readonly name: string
  readonly description: string
  readonly configuration: any
  readonly hasWorkflows: boolean

  private _loading = false
  private _value: any

  constructor(
    key: string,
    ui: AttributeUiType,
    list: boolean,
    name: string,
    description: string,
    configuration: any,
    hasWorkflows: boolean = false,
  ) {
    this.key = key
    this.ui = ui
    this.list = list
    this.name = name
    this.description = description
    this.configuration = configuration
    this.hasWorkflows = hasWorkflows
    this._value = null
  }

  get loading(): boolean {
    return this._loading
  }

  set loading(loading: boolean) {
    this._loading = loading
  }

  get value(): any {
    return this._value
  }

  set value(value: any) {
    if (typeof value === 'string' && value.length === 0) {
      this._value = undefined
    } else {
      this._value = value
    }
  }

  async setSupplementaryValue(
    client: BoscaClient<any>,
    metadataId: string,
    supplementaryKey: string,
  ) {
    switch (this.ui) {
      case AttributeUiType.Textarea:
      case AttributeUiType.Input: {
        this.value = await client.metadata.getSupplementaryText(
          metadataId,
          supplementaryKey,
        )
        if (this.value) {
          this.loading = false
        }
        break
      }
      case AttributeUiType.Collection: {
        const collections = await client.metadata.getSupplementaryJson(
          metadataId,
          supplementaryKey,
        )
        if (collections?.collections) {
          this.value = collections?.collections
          this.loading = false
        }
        break
      }
      default:
        this.loading = false
        break
    }
  }
}

export function newAttributeState(
  attribute: DocumentTemplateAttribute,
): AttributeState {
  return new AttributeState(
    attribute.key,
    attribute.ui,
    attribute.list,
    attribute.name,
    attribute.description,
    attribute.configuration,
    attribute.workflows && attribute.workflows.length > 0,
  )
}
