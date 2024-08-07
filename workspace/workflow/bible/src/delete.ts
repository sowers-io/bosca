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

import { Activity, ActivityJobExecutor, Downloader, Job } from '@bosca/workflow-activities-api'
import { USXProcessor } from '@bosca/bible-processor'
import { ContentService, FindMetadataRequest, IdRequest, WorkflowJob } from '@bosca/protobufs'
import { useServiceAccountClient } from '@bosca/common'

export class DeleteBibleActivity extends Activity {
  readonly downloader: Downloader

  constructor(downloader: Downloader) {
    super()
    this.downloader = downloader
  }

  get id(): string {
    return 'bible.delete'
  }

  newJobExecutor(job: Job, definition: WorkflowJob): ActivityJobExecutor<any> {
    return new Executor(this, job, definition)
  }
}

class Executor extends ActivityJobExecutor<DeleteBibleActivity> {

  async execute() {
    const file = await this.activity.downloader.download(this.definition)
    try {
      const processor = new USXProcessor()
      await processor.process(file)
      const metadata = processor.metadata!
      const contentService = useServiceAccountClient(ContentService)
      const metadatas = await contentService.findMetadata(new FindMetadataRequest({
        attributes: {
          'bible.system.id': metadata.identification.systemId.id,
        },
      }))
      for (const metadata of metadatas.metadata) {
        await contentService.deleteMetadata(new IdRequest({ id: metadata.id }))
      }
      const collections = await contentService.findCollection(new FindMetadataRequest({
        attributes: {
          'bible.system.id': metadata.identification.systemId.id,
        },
      }))
      for (const collection of collections.collections) {
        await contentService.deleteCollection(new IdRequest({ id: collection.id }))
      }
    } finally {
      await this.activity.downloader.cleanup(file)
    }
  }
}