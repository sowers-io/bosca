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

import { ObjectStore } from './objectstore'
import { Metadata, MetadataSupplementary, SignedUrl, SignedUrlHeader } from '@bosca/protobufs'
import { Code, ConnectError } from '@connectrpc/connect'
import { DeleteObjectCommand, GetObjectCommand, PutObjectCommand, S3Client } from '@aws-sdk/client-s3'
import { getSignedUrl } from '@aws-sdk/s3-request-presigner'
import { S3RequestPresigner } from '@aws-sdk/s3-request-presigner'

export class S3ObjectStore implements ObjectStore {
  private readonly client: S3Client
  private readonly signer: S3RequestPresigner
  private readonly bucket: string

  constructor() {
    this.bucket = process.env.BOSCA_S3_BUCKET || 'bosca'
    this.client = new S3Client({
      forcePathStyle: process.env.BOSCA_S3_FORCE_PATH_STYLE === 'true',
      endpoint: process.env.BOSCA_S3_ENDPOINT!,
      tls: process.env.BOSCA_S3_ENDPOINT!.endsWith('https://'),
      region: process.env.BOSCA_S3_REGION || 'us-east-1',
      credentials: {
        accessKeyId: process.env.BOSCA_S3_ACCESS_KEY_ID!,
        secretAccessKey: process.env.BOSCA_S3_SECRET_ACCESS_KEY!,
      },
    })
    this.signer = new S3RequestPresigner({
      ...this.client.config,
    })
  }

  private getId(metadata: Metadata, supplementary: MetadataSupplementary | null): string {
    if (supplementary) {
      return metadata.id + '.' + metadata.version + '.' + supplementary.key
    }
    return metadata.id + '.' + metadata.version
  }

  async createUploadUrl(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<SignedUrl> {
    if (!metadata.contentLength) {
      throw new ConnectError('metadata does not have a content length', Code.FailedPrecondition)
    }
    const id = this.getId(metadata, supplementary)
    const length = Number(supplementary ? supplementary.contentLength : metadata.contentLength)
    const command = new PutObjectCommand({
      Bucket: this.bucket,
      Key: id,
      ContentType: metadata.contentType,
      ContentLength: length,
    })
    const url = await getSignedUrl(this.client, command, {
      expiresIn: 3600,
    })
    const headers: SignedUrlHeader[] = [
      new SignedUrlHeader({ name: 'Content-Type', value: metadata.contentType }),
      new SignedUrlHeader({ name: 'Content-Length', value: length.toString() }),
    ]
    return new SignedUrl({
      id: id,
      headers: headers,
      method: 'PUT',
      url: url,
    })
  }

  async createDownloadUrl(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<SignedUrl> {
    const id = this.getId(metadata, supplementary)
    const command = new GetObjectCommand({
      Bucket: this.bucket,
      Key: id,
    })
    const url = await getSignedUrl(this.client, command, {
      expiresIn: 3600,
    })
    // const url = await this.client.presignedGetObject(this.bucket, id, 5 * 60)
    return new SignedUrl({ id: id, method: 'GET', url: url })
  }

  async delete(metadata: Metadata, supplementary: MetadataSupplementary | null): Promise<void> {
    const id = this.getId(metadata, supplementary)
    // await this.client.removeObject(this.bucket, id)
    await this.client.send(new DeleteObjectCommand({ Bucket: this.bucket, Key: id }))
  }
}
