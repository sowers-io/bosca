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

package metadata

import (
	"bosca.io/pkg/workers/common"
	"bosca.io/pkg/workers/metadata/processor"
	"go.temporal.io/sdk/client"
	"go.temporal.io/sdk/worker"
)

const TaskQueue = "metadata"

func NewWorker(client client.Client) worker.Worker {
	w := worker.New(client, TaskQueue, worker.Options{})
	w.RegisterWorkflow(ProcessMetadata)
	w.RegisterWorkflow(ProcessTraits)

	w.RegisterActivity(common.GetMetadata)
	w.RegisterActivity(processor.GetTraitWorkflows)
	w.RegisterActivity(processor.CompleteTransition)
	w.RegisterActivity(processor.TransitionTo)
	return w
}
