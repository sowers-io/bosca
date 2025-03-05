import { CollectionType } from '~/lib/graphql/graphql'
import { type BoscaClient } from '~/lib/bosca/client'
import type { CollectionAndItems } from '~/lib/bosca/contentcollection.ts'
import { toast } from '~/components/ui/toast/index'

export class Uploader {
  private assets: CollectionAndItems | null = null
  private assetsDirectoryId: string | null | undefined = null

  constructor(private readonly client: BoscaClient<any>) {
    if (import.meta.client) {
      this.initialize()
    }
  }

  async initialize() {
    const root = await this.client.collections.list(
      '00000000-0000-0000-0000-000000000000',
    )
    this.assetsDirectoryId = root!.items.find((item) => item.name === 'Assets')
      ?.id
    if (!this.assetsDirectoryId) {
      this.assetsDirectoryId = await this.client.collections.add({
        name: 'Assets',
        collectionType: CollectionType.Folder,
        parentCollectionId: '00000000-0000-0000-0000-000000000000',
      })
      await this.client.collections.setReady(this.assetsDirectoryId)
    }
    this.assets = await this.client.collections.list(this.assetsDirectoryId)
  }

  isAssetCollection(id: string): boolean {
    if (id === this.assetsDirectoryId) return true
    const item = this.assets?.items?.find((c) => c.id == id) || null
    return item != null
  }

  async upload(files: File[] | null): Promise<string[]> {
    if (!files || !this.assets) return []
    const fileTypes: { [type: string]: Array<File> } = {}

    for (const file of files) {
      if (!fileTypes[file.type]) {
        fileTypes[file.type] = []
      }
      fileTypes[file.type].push(file)
    }

    const childByName: { [name: string]: string } = {}
    for (const child of this.assets!.items) {
      childByName[child.name] = child.id
    }

    const uploadIds = []
    for (const type in fileTypes) {
      const files = fileTypes[type]
      let collectionName: string | null = null
      if (type.startsWith('image/')) {
        collectionName = 'Images'
      } else if (type.startsWith('video/')) {
        collectionName = 'Videos'
      } else if (type.startsWith('audio/')) {
        collectionName = 'Audio'
      } else if (type.startsWith('application/pdf')) {
        collectionName = 'PDFs'
      } else {
        console.error('Unsupported file type: ' + type)
      }
      if (!collectionName) {
        toast({
          title: 'Unsupported file type: ' + type,
        })
        continue
      }
      let collectionId = childByName[collectionName]
      if (!collectionId) {
        collectionId = await this.client.collections.add({
          parentCollectionId: this.assetsDirectoryId,
          collectionType: CollectionType.Folder,
          name: collectionName,
        })
        await this.client.collections.setReady(collectionId)
      }
      const ids = await this.client.metadata.addFiles(collectionId, files)
      for (const id of ids) {
        await this.client.metadata.setReady(id)
      }
      uploadIds.push(...ids)
    }

    return uploadIds
  }
}
