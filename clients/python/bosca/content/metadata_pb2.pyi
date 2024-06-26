from google.protobuf import timestamp_pb2 as _timestamp_pb2
from google.protobuf.internal import containers as _containers
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from typing import ClassVar as _ClassVar, Iterable as _Iterable, Mapping as _Mapping, Optional as _Optional, Union as _Union

DESCRIPTOR: _descriptor.FileDescriptor

class Metadata(_message.Message):
    __slots__ = ("default_id", "id", "name", "content_type", "source", "language_tag", "content_length", "trait_ids", "category_ids", "tags", "attributes", "created", "modified", "workflow_state_id", "workflow_state_pending_id")
    class AttributesEntry(_message.Message):
        __slots__ = ("key", "value")
        KEY_FIELD_NUMBER: _ClassVar[int]
        VALUE_FIELD_NUMBER: _ClassVar[int]
        key: str
        value: str
        def __init__(self, key: _Optional[str] = ..., value: _Optional[str] = ...) -> None: ...
    DEFAULT_ID_FIELD_NUMBER: _ClassVar[int]
    ID_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    SOURCE_FIELD_NUMBER: _ClassVar[int]
    LANGUAGE_TAG_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    TRAIT_IDS_FIELD_NUMBER: _ClassVar[int]
    CATEGORY_IDS_FIELD_NUMBER: _ClassVar[int]
    TAGS_FIELD_NUMBER: _ClassVar[int]
    ATTRIBUTES_FIELD_NUMBER: _ClassVar[int]
    CREATED_FIELD_NUMBER: _ClassVar[int]
    MODIFIED_FIELD_NUMBER: _ClassVar[int]
    WORKFLOW_STATE_ID_FIELD_NUMBER: _ClassVar[int]
    WORKFLOW_STATE_PENDING_ID_FIELD_NUMBER: _ClassVar[int]
    default_id: str
    id: str
    name: str
    content_type: str
    source: str
    language_tag: str
    content_length: int
    trait_ids: _containers.RepeatedScalarFieldContainer[str]
    category_ids: _containers.RepeatedScalarFieldContainer[str]
    tags: _containers.RepeatedScalarFieldContainer[str]
    attributes: _containers.ScalarMap[str, str]
    created: _timestamp_pb2.Timestamp
    modified: _timestamp_pb2.Timestamp
    workflow_state_id: str
    workflow_state_pending_id: str
    def __init__(self, default_id: _Optional[str] = ..., id: _Optional[str] = ..., name: _Optional[str] = ..., content_type: _Optional[str] = ..., source: _Optional[str] = ..., language_tag: _Optional[str] = ..., content_length: _Optional[int] = ..., trait_ids: _Optional[_Iterable[str]] = ..., category_ids: _Optional[_Iterable[str]] = ..., tags: _Optional[_Iterable[str]] = ..., attributes: _Optional[_Mapping[str, str]] = ..., created: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., modified: _Optional[_Union[_timestamp_pb2.Timestamp, _Mapping]] = ..., workflow_state_id: _Optional[str] = ..., workflow_state_pending_id: _Optional[str] = ...) -> None: ...

class AddMetadataRequest(_message.Message):
    __slots__ = ("collection", "metadata")
    COLLECTION_FIELD_NUMBER: _ClassVar[int]
    METADATA_FIELD_NUMBER: _ClassVar[int]
    collection: str
    metadata: Metadata
    def __init__(self, collection: _Optional[str] = ..., metadata: _Optional[_Union[Metadata, _Mapping]] = ...) -> None: ...

class Metadatas(_message.Message):
    __slots__ = ("metadata",)
    METADATA_FIELD_NUMBER: _ClassVar[int]
    metadata: _containers.RepeatedCompositeFieldContainer[Metadata]
    def __init__(self, metadata: _Optional[_Iterable[_Union[Metadata, _Mapping]]] = ...) -> None: ...

class AddMetadataRelationshipRequest(_message.Message):
    __slots__ = ("metadata_id1", "metadata_id2", "relationship")
    METADATA_ID1_FIELD_NUMBER: _ClassVar[int]
    METADATA_ID2_FIELD_NUMBER: _ClassVar[int]
    RELATIONSHIP_FIELD_NUMBER: _ClassVar[int]
    metadata_id1: str
    metadata_id2: str
    relationship: str
    def __init__(self, metadata_id1: _Optional[str] = ..., metadata_id2: _Optional[str] = ..., relationship: _Optional[str] = ...) -> None: ...

class AddMetadataTraitRequest(_message.Message):
    __slots__ = ("metadata_id", "trait_id")
    METADATA_ID_FIELD_NUMBER: _ClassVar[int]
    TRAIT_ID_FIELD_NUMBER: _ClassVar[int]
    metadata_id: str
    trait_id: str
    def __init__(self, metadata_id: _Optional[str] = ..., trait_id: _Optional[str] = ...) -> None: ...

class AddSupplementaryRequest(_message.Message):
    __slots__ = ("id", "type", "name", "content_type", "content_length")
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    NAME_FIELD_NUMBER: _ClassVar[int]
    CONTENT_TYPE_FIELD_NUMBER: _ClassVar[int]
    CONTENT_LENGTH_FIELD_NUMBER: _ClassVar[int]
    id: str
    type: str
    name: str
    content_type: str
    content_length: int
    def __init__(self, id: _Optional[str] = ..., type: _Optional[str] = ..., name: _Optional[str] = ..., content_type: _Optional[str] = ..., content_length: _Optional[int] = ...) -> None: ...

class SupplementaryIdRequest(_message.Message):
    __slots__ = ("id", "type")
    ID_FIELD_NUMBER: _ClassVar[int]
    TYPE_FIELD_NUMBER: _ClassVar[int]
    id: str
    type: str
    def __init__(self, id: _Optional[str] = ..., type: _Optional[str] = ...) -> None: ...
