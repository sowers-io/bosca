//
// Copyright 2024 Sowers, LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// @generated by protoc-gen-connect-es v1.4.0 with parameter "target=ts,import_extension=none"
// @generated from file bosca/workflow/queue_service.proto (package bosca.workflow, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import { WorkflowJobId } from "../content/workflows_pb";
import { WorkflowJobInstance } from "./workflows_pb";
import { MethodKind } from "@bufbuild/protobuf";
import { WorkflowEnqueueRequest, WorkflowEnqueueResponse } from "./execution_context_pb";

/**
 * @generated from service bosca.workflow.WorkflowQueueService
 */
export const WorkflowQueueService = {
  typeName: "bosca.workflow.WorkflowQueueService",
  methods: {
    /**
     * @generated from rpc bosca.workflow.WorkflowQueueService.GetJob
     */
    getJob: {
      name: "GetJob",
      I: WorkflowJobId,
      O: WorkflowJobInstance,
      kind: MethodKind.Unary,
    },
    /**
     * @generated from rpc bosca.workflow.WorkflowQueueService.Enqueue
     */
    enqueue: {
      name: "Enqueue",
      I: WorkflowEnqueueRequest,
      O: WorkflowEnqueueResponse,
      kind: MethodKind.Unary,
    },
  }
} as const;

