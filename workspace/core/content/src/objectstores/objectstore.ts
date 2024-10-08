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

import { Metadata, MetadataSupplementary, SignedUrl } from '@bosca/protobufs'

export interface ObjectStore {
  createUploadUrl(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<SignedUrl>

  createDownloadUrl(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<SignedUrl>

  delete(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<void>
}
