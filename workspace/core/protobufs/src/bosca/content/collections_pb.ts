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

// @generated by protoc-gen-es v1.10.0 with parameter "target=ts,import_extension=none"
// @generated from file bosca/content/collections.proto (package bosca.content, syntax proto3)
/* eslint-disable */
// @ts-nocheck

import type { BinaryReadOptions, FieldList, JsonReadOptions, JsonValue, PartialMessage, PlainMessage } from "@bufbuild/protobuf";
import { Message, proto3, Struct, Timestamp } from "@bufbuild/protobuf";
import { Metadata } from "./metadata_pb";

/**
 * @generated from enum bosca.content.CollectionType
 */
export enum CollectionType {
  /**
   * @generated from enum value: standard = 0;
   */
  standard = 0,

  /**
   * @generated from enum value: folder = 1;
   */
  folder = 1,

  /**
   * @generated from enum value: root = 2;
   */
  root = 2,
}
// Retrieve enum metadata with: proto3.getEnumType(CollectionType)
proto3.util.setEnumType(CollectionType, "bosca.content.CollectionType", [
  { no: 0, name: "standard" },
  { no: 1, name: "folder" },
  { no: 2, name: "root" },
]);

/**
 * @generated from message bosca.content.AddCollectionRequest
 */
export class AddCollectionRequest extends Message<AddCollectionRequest> {
  /**
   * @generated from field: string parent = 1;
   */
  parent = "";

  /**
   * @generated from field: bosca.content.Collection collection = 2;
   */
  collection?: Collection;

  constructor(data?: PartialMessage<AddCollectionRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.AddCollectionRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "parent", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "collection", kind: "message", T: Collection },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): AddCollectionRequest {
    return new AddCollectionRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): AddCollectionRequest {
    return new AddCollectionRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): AddCollectionRequest {
    return new AddCollectionRequest().fromJsonString(jsonString, options);
  }

  static equals(a: AddCollectionRequest | PlainMessage<AddCollectionRequest> | undefined, b: AddCollectionRequest | PlainMessage<AddCollectionRequest> | undefined): boolean {
    return proto3.util.equals(AddCollectionRequest, a, b);
  }
}

/**
 * @generated from message bosca.content.AddCollectionsRequest
 */
export class AddCollectionsRequest extends Message<AddCollectionsRequest> {
  /**
   * @generated from field: repeated bosca.content.AddCollectionRequest collections = 1;
   */
  collections: AddCollectionRequest[] = [];

  constructor(data?: PartialMessage<AddCollectionsRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.AddCollectionsRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "collections", kind: "message", T: AddCollectionRequest, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): AddCollectionsRequest {
    return new AddCollectionsRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): AddCollectionsRequest {
    return new AddCollectionsRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): AddCollectionsRequest {
    return new AddCollectionsRequest().fromJsonString(jsonString, options);
  }

  static equals(a: AddCollectionsRequest | PlainMessage<AddCollectionsRequest> | undefined, b: AddCollectionsRequest | PlainMessage<AddCollectionsRequest> | undefined): boolean {
    return proto3.util.equals(AddCollectionsRequest, a, b);
  }
}

/**
 * @generated from message bosca.content.AddCollectionItemRequest
 */
export class AddCollectionItemRequest extends Message<AddCollectionItemRequest> {
  /**
   * @generated from field: string collection_id = 1;
   */
  collectionId = "";

  /**
   * @generated from oneof bosca.content.AddCollectionItemRequest.item_id
   */
  itemId: {
    /**
     * @generated from field: string child_collection_id = 2;
     */
    value: string;
    case: "childCollectionId";
  } | {
    /**
     * @generated from field: string child_metadata_id = 3;
     */
    value: string;
    case: "childMetadataId";
  } | { case: undefined; value?: undefined } = { case: undefined };

  constructor(data?: PartialMessage<AddCollectionItemRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.AddCollectionItemRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "collection_id", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 2, name: "child_collection_id", kind: "scalar", T: 9 /* ScalarType.STRING */, oneof: "item_id" },
    { no: 3, name: "child_metadata_id", kind: "scalar", T: 9 /* ScalarType.STRING */, oneof: "item_id" },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): AddCollectionItemRequest {
    return new AddCollectionItemRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): AddCollectionItemRequest {
    return new AddCollectionItemRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): AddCollectionItemRequest {
    return new AddCollectionItemRequest().fromJsonString(jsonString, options);
  }

  static equals(a: AddCollectionItemRequest | PlainMessage<AddCollectionItemRequest> | undefined, b: AddCollectionItemRequest | PlainMessage<AddCollectionItemRequest> | undefined): boolean {
    return proto3.util.equals(AddCollectionItemRequest, a, b);
  }
}

/**
 * @generated from message bosca.content.Collection
 */
export class Collection extends Message<Collection> {
  /**
   * @generated from field: string id = 2;
   */
  id = "";

  /**
   * @generated from field: string name = 3;
   */
  name = "";

  /**
   * @generated from field: bosca.content.CollectionType type = 5;
   */
  type = CollectionType.standard;

  /**
   * @generated from field: repeated string trait_ids = 11;
   */
  traitIds: string[] = [];

  /**
   * @generated from field: repeated string category_ids = 12;
   */
  categoryIds: string[] = [];

  /**
   * @generated from field: repeated string labels = 13;
   */
  labels: string[] = [];

  /**
   * @generated from field: map<string, string> attributes = 14;
   */
  attributes: { [key: string]: string } = {};

  /**
   * @generated from field: google.protobuf.Timestamp created = 20;
   */
  created?: Timestamp;

  /**
   * @generated from field: google.protobuf.Timestamp modified = 21;
   */
  modified?: Timestamp;

  /**
   * @generated from field: google.protobuf.Struct metadata = 33;
   */
  metadata?: Struct;

  constructor(data?: PartialMessage<Collection>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.Collection";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 2, name: "id", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 3, name: "name", kind: "scalar", T: 9 /* ScalarType.STRING */ },
    { no: 5, name: "type", kind: "enum", T: proto3.getEnumType(CollectionType) },
    { no: 11, name: "trait_ids", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
    { no: 12, name: "category_ids", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
    { no: 13, name: "labels", kind: "scalar", T: 9 /* ScalarType.STRING */, repeated: true },
    { no: 14, name: "attributes", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "scalar", T: 9 /* ScalarType.STRING */} },
    { no: 20, name: "created", kind: "message", T: Timestamp },
    { no: 21, name: "modified", kind: "message", T: Timestamp },
    { no: 33, name: "metadata", kind: "message", T: Struct },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Collection {
    return new Collection().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Collection {
    return new Collection().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Collection {
    return new Collection().fromJsonString(jsonString, options);
  }

  static equals(a: Collection | PlainMessage<Collection> | undefined, b: Collection | PlainMessage<Collection> | undefined): boolean {
    return proto3.util.equals(Collection, a, b);
  }
}

/**
 * @generated from message bosca.content.Collections
 */
export class Collections extends Message<Collections> {
  /**
   * @generated from field: repeated bosca.content.Collection collections = 1;
   */
  collections: Collection[] = [];

  constructor(data?: PartialMessage<Collections>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.Collections";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "collections", kind: "message", T: Collection, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): Collections {
    return new Collections().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): Collections {
    return new Collections().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): Collections {
    return new Collections().fromJsonString(jsonString, options);
  }

  static equals(a: Collections | PlainMessage<Collections> | undefined, b: Collections | PlainMessage<Collections> | undefined): boolean {
    return proto3.util.equals(Collections, a, b);
  }
}

/**
 * @generated from message bosca.content.CollectionItems
 */
export class CollectionItems extends Message<CollectionItems> {
  /**
   * @generated from field: repeated bosca.content.CollectionItem items = 1;
   */
  items: CollectionItem[] = [];

  constructor(data?: PartialMessage<CollectionItems>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.CollectionItems";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "items", kind: "message", T: CollectionItem, repeated: true },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): CollectionItems {
    return new CollectionItems().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): CollectionItems {
    return new CollectionItems().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): CollectionItems {
    return new CollectionItems().fromJsonString(jsonString, options);
  }

  static equals(a: CollectionItems | PlainMessage<CollectionItems> | undefined, b: CollectionItems | PlainMessage<CollectionItems> | undefined): boolean {
    return proto3.util.equals(CollectionItems, a, b);
  }
}

/**
 * @generated from message bosca.content.CollectionItem
 */
export class CollectionItem extends Message<CollectionItem> {
  /**
   * @generated from oneof bosca.content.CollectionItem.Item
   */
  Item: {
    /**
     * @generated from field: bosca.content.Metadata metadata = 1;
     */
    value: Metadata;
    case: "metadata";
  } | {
    /**
     * @generated from field: bosca.content.Collection collection = 3;
     */
    value: Collection;
    case: "collection";
  } | { case: undefined; value?: undefined } = { case: undefined };

  constructor(data?: PartialMessage<CollectionItem>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.CollectionItem";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "metadata", kind: "message", T: Metadata, oneof: "Item" },
    { no: 3, name: "collection", kind: "message", T: Collection, oneof: "Item" },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): CollectionItem {
    return new CollectionItem().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): CollectionItem {
    return new CollectionItem().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): CollectionItem {
    return new CollectionItem().fromJsonString(jsonString, options);
  }

  static equals(a: CollectionItem | PlainMessage<CollectionItem> | undefined, b: CollectionItem | PlainMessage<CollectionItem> | undefined): boolean {
    return proto3.util.equals(CollectionItem, a, b);
  }
}

/**
 * @generated from message bosca.content.FindCollectionRequest
 */
export class FindCollectionRequest extends Message<FindCollectionRequest> {
  /**
   * @generated from field: map<string, string> attributes = 1;
   */
  attributes: { [key: string]: string } = {};

  constructor(data?: PartialMessage<FindCollectionRequest>) {
    super();
    proto3.util.initPartial(data, this);
  }

  static readonly runtime: typeof proto3 = proto3;
  static readonly typeName = "bosca.content.FindCollectionRequest";
  static readonly fields: FieldList = proto3.util.newFieldList(() => [
    { no: 1, name: "attributes", kind: "map", K: 9 /* ScalarType.STRING */, V: {kind: "scalar", T: 9 /* ScalarType.STRING */} },
  ]);

  static fromBinary(bytes: Uint8Array, options?: Partial<BinaryReadOptions>): FindCollectionRequest {
    return new FindCollectionRequest().fromBinary(bytes, options);
  }

  static fromJson(jsonValue: JsonValue, options?: Partial<JsonReadOptions>): FindCollectionRequest {
    return new FindCollectionRequest().fromJson(jsonValue, options);
  }

  static fromJsonString(jsonString: string, options?: Partial<JsonReadOptions>): FindCollectionRequest {
    return new FindCollectionRequest().fromJsonString(jsonString, options);
  }

  static equals(a: FindCollectionRequest | PlainMessage<FindCollectionRequest> | undefined, b: FindCollectionRequest | PlainMessage<FindCollectionRequest> | undefined): boolean {
    return proto3.util.equals(FindCollectionRequest, a, b);
  }
}

