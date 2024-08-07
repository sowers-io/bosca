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

import { Activity, ActivityJobExecutor, Job } from '@bosca/workflow-activities-api'
import { ContentService, SetWorkflowStateCompleteRequest, SetWorkflowStateRequest, WorkflowJob } from '@bosca/protobufs'
import { useServiceAccountClient } from '@bosca/common'

export class TransitionToActivity extends Activity {
  get id(): string {
    return 'metadata.transition.to'
  }

  newJobExecutor(job: Job, definition: WorkflowJob): ActivityJobExecutor<any> {
    return new Executor(this, job, definition)
  }
}

class Executor extends ActivityJobExecutor<TransitionToActivity> {
  async execute() {
    const contentService = useServiceAccountClient(ContentService)
    const state = this.definition.activity?.configuration['state']
    const status = this.definition.activity?.configuration['status']
    await contentService.setWorkflowStateComplete(
      new SetWorkflowStateCompleteRequest({
        metadataId: this.definition.metadataId,
        status: status,
      }),
    )
    await contentService.setWorkflowState(
      new SetWorkflowStateRequest({
        metadataId: this.definition.metadataId,
        stateId: state,
        status: status,
        immediate: true,
      }),
    )
  }
}
