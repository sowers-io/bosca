/*
 * Copyright 2024 Sowers, LLC
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

import { USXProcessor, BibleMetadata, Book } from '@bosca/bible-processor'
import {
  AddCollectionRequest,
  AddMetadataRequest,
  Collection,
  Metadata,
  ContentService,
  IdRequest,
  Source,
  WorkflowJob,
  FindCollectionRequest,
  AddCollectionItemRequest,
} from '@bosca/protobufs'
import {
  Job,
  Activity,
  ActivityJobExecutor,
  Downloader,
  getCollection,
  toArrayBuffer,
  addCollections,
  addMetadatas,
  uploadAll,
  uploadSupplementary,
} from '@bosca/workflow-activities-api'
import { protoInt64 } from '@bufbuild/protobuf'
import { useServiceAccountClient } from '@bosca/common'
import { HtmlContext } from '@bosca/bible-processor'

export class ProcessBibleActivity extends Activity {
  readonly downloader: Downloader

  constructor(downloader: Downloader) {
    super()
    this.downloader = downloader
  }

  get id(): string {
    return 'bible.process'
  }

  newJobExecutor(job: Job, definition: WorkflowJob): ActivityJobExecutor<any> {
    return new Executor(this, job, definition)
  }
}

class Executor extends ActivityJobExecutor<ProcessBibleActivity> {

  private async createBibleCollection(metadata: BibleMetadata): Promise<Collection> {
    const version = await this.findNextVersion(metadata)
    const service = useServiceAccountClient(ContentService)
    const addResponse = await service.addCollection(
      new AddCollectionRequest({
        collection: new Collection({
          name: metadata.identification.nameLocal,
          attributes: {
            'bible.type': 'bible',
            'bible.language': metadata.language.iso,
            'bible.system.id': metadata.identification.systemId.id,
            'bible.abbreviation': metadata.identification.abbreviationLocal,
            'bible.version': version.toString(),
          },
        }),
      }),
    )
    const collection = await getCollection(new IdRequest({ id: addResponse.id }))
    service.addCollectionItem(new AddCollectionItemRequest({
      collectionId: collection.id,
      itemId: {
        case: 'childMetadataId',
        value: this.definition.metadataId!,
      },
    }))
    return collection
  }

  private async createBookCollections(
    source: Source,
    metadata: BibleMetadata,
    bible: Collection,
    books: Book[],
  ): Promise<Collection[]> {
    const addCollectionRequests: AddCollectionRequest[] = []
    const addMetadatasRequests: AddMetadataRequest[] = []
    const buffers: ArrayBuffer[] = []
    let order = 0

    // build bulk requests
    for (const book of books) {
      const attributes = {
        'bible.type': 'book',
        'bible.language': metadata.language.iso,
        'bible.version': bible.attributes['bible.version'],
        'bible.system.id': bible.attributes['bible.system.id'],
        'bible.abbreviation': bible.attributes['bible.abbreviation'],
        'bible.book.usfm': book.usfm,
        'bible.book.order': order.toString(),
      }
      addCollectionRequests.push(
        new AddCollectionRequest({
          parent: bible.id,
          collection: new Collection({
            name: book.name.long + ' Chapters',
            attributes: attributes,
          }),
        }),
      )
      const buffer = toArrayBuffer(book.raw)
      buffers.push(buffer)
      addMetadatasRequests.push(
        new AddMetadataRequest({
          collection: bible.id,
          metadata: new Metadata({
            name: book.name.long,
            contentType: 'bible/usx-book',
            languageTag: metadata.language.iso,
            contentLength: protoInt64.parse(buffer.byteLength),
            attributes: attributes,
            sourceId: source.id,
            traitIds: ['bible.usx.book'],
          }),
        }),
      )
      order++
    }

    // create collections
    const addCollectionResponses = await addCollections(addCollectionRequests)

    // create metadata
    const responses = await addMetadatas(addMetadatasRequests, buffers)
    for (const id of responses.id) {
      if (id.error && id.error != 'name must be unique') {
        throw new Error(id.error)
      }
    }

    // fetch created collections
    const collections: Collection[] = []
    for (const addResponse of addCollectionResponses.id) {
      if (addResponse.error && addResponse.error != 'name must be unique') {
        throw new Error(addResponse.error)
      }
      // TODO: Add bulk getCollections
      const collection = await getCollection(new IdRequest({ id: addResponse.id }))
      collections.push(collection)
    }
    return collections
  }

  private async createChapters(source: Source, metadata: BibleMetadata, bookCollection: Collection, book: Book) {
    const requests: AddMetadataRequest[] = []
    const usxChapters: ArrayBuffer[] = []
    const htmlChapters: ArrayBuffer[] = []
    let order = 0
    for (const chapter of book.chapters) {
      const buffer = toArrayBuffer(book.raw.substring(chapter.position.start, chapter.position.end))
      usxChapters.push(buffer)
      const chapterHtml = toArrayBuffer(chapter.toHtml(new HtmlContext(false, 0, true)))
      htmlChapters.push(chapterHtml)
      requests.push(
        new AddMetadataRequest({
          collection: bookCollection.id,
          metadata: new Metadata({
            name: book.name.long + ' ' + chapter.number,
            contentType: 'bible/usx-chapter',
            contentLength: protoInt64.parse(buffer.byteLength),
            languageTag: metadata.language.iso,
            attributes: {
              'bible.type': 'chapter',
              'bible.language': metadata.language.iso,
              'bible.version': bookCollection.attributes['bible.version'],
              'bible.system.id': bookCollection.attributes['bible.system.id'],
              'bible.abbreviation': bookCollection.attributes['bible.abbreviation'],
              'bible.book.usfm': book.usfm,
              'bible.chapter.usfm': chapter.usfm,
              'bible.book.order': bookCollection.attributes['bible.book.order'],
              'bible.chapter.order': (order++).toString(),
            },
            sourceId: source.id,
            traitIds: ['bible.usx.chapter'],
          }),
        }),
      )
    }
    const responses = await addMetadatas(requests)
    for (let i = 0; i < responses.id.length; i++) {
      await uploadSupplementary(
        responses.id[i].id,
        'Chapter HTML',
        'text/html',
        book.chapters[i].usfm,
        source.id,
        undefined,
        undefined,
        htmlChapters[i],
      )
    }
    await uploadAll(responses, usxChapters)
  }

  private async findNextVersion(metadata: BibleMetadata): Promise<number> {
    let version = 1
    const collections = await useServiceAccountClient(ContentService).findCollection(
      new FindCollectionRequest({
        attributes: {
          'bible.type': 'bible',
          'bible.system.id': metadata.identification.systemId.id,
        },
      }),
    )
    for (const collection of collections.collections) {
      version = Math.max(parseInt(collection.attributes['bible.version']), version)
    }
    return version
  }

  async execute() {
    const source = await useServiceAccountClient(ContentService).getSource(new IdRequest({ id: 'workflow' }))
    const file = await this.activity.downloader.download(this.definition)
    try {
      const processor = new USXProcessor()
      await processor.process(file)

      const metadata = processor.metadata!
      const bibleCollection = await this.createBibleCollection(metadata)
      const bookCollections = await this.createBookCollections(source, metadata, bibleCollection, processor.books)

      for (let bookIndex = 0; bookIndex < processor.books.length; bookIndex++) {
        const book = processor.books[bookIndex]
        const collection = bookCollections[bookIndex]
        await this.createChapters(source, metadata, collection, book)
      }
    } finally {
      await this.activity.downloader.cleanup(file)
    }
  }
}
