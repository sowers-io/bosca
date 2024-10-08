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

import {
  Permissions,
  Permission,
  PermissionAction,
  PermissionObjectType,
  PermissionSubjectType,
} from '@bosca/protobufs'
import { Subject } from '../authentication/subject_finder'
import { Code, ConnectError } from '@connectrpc/connect'

export enum SubjectType {
  user = 'user',
  group = 'group',
  serviceaccount = 'serviceaccount',
}

export class PermissionError extends ConnectError {
  constructor(message: string) {
    super(message, Code.PermissionDenied)
  }
}

export interface PermissionManager {
  bulkCheck(
    subject: Subject,
    objectType: PermissionObjectType,
    resourceId: string[],
    action: PermissionAction
  ): Promise<string[]>

  checkWithError(
    subject: Subject,
    objectType: PermissionObjectType,
    resourceId: string,
    action: PermissionAction
  ): Promise<void>

  checkWithSubjectIdError(
    subjectType: PermissionSubjectType,
    subjectId: string,
    objectType: PermissionObjectType,
    resourceId: string,
    action: PermissionAction
  ): Promise<void>

  createRelationships(objectType: PermissionObjectType, permissions: Permission[]): Promise<void>

  deleteRelationships(objectType: PermissionObjectType, permissions: Permission[]): Promise<void>

  createRelationship(objectType: PermissionObjectType, permission: Permission): Promise<void>

  waitForPermissions(objectType: PermissionObjectType, permissions: Permission[]): Promise<void>

  getPermissions(objectType: PermissionObjectType, resourceId: string): Promise<Permissions>
}
