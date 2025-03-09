# Metadata

<primary-label ref="bosca"/>
<secondary-label ref="beta"/>

Metadata is the foundation of content management in Bosca. It can reference internal or external content, serve as a
container for content, all of which also serves for Workflow operations. Whether dealing with structured or
unstructured data, Metadata provides the framework to manage and deliver this content effectively.

## Core Metadata Features

### Language Variants

Metadata supports language variants through the parent-child relationship model. A parent metadata item can have multiple
child variants, each representing the same content in different languages. This is implemented through the `parent_id` field,
which links variant metadata to its parent.

Key characteristics of language variants:
- Each variant has its own unique ID but shares a common parent ID
- Variants are identified by their `language_tag` field
- The `metadata_type` field distinguishes between standard metadata and variants
- Workflows can be configured to automatically create or update variants when the parent content changes

### Labels and Attributes

Metadata includes several ways to categorize, describe, and enhance content:

#### Labels
The `labels` field allows for flexible tagging of metadata with arbitrary strings. Labels can be used for:
- Content categorization
- Filtering and searching
- Feature flagging
- Workflow triggers

#### Attributes
Metadata supports three types of attributes:

1. **General Attributes** (`attributes` field): Custom JSON data specific to the content
2. **System Attributes** (`system_attributes` field): System-managed properties that control behavior
3. **Item Attributes** (`item_attributes` field): Properties specific to the content item type

These attribute systems provide a flexible way to extend metadata with custom properties without changing the core schema.

### External Source Integration

Metadata can link to external content sources through:
- `source_id`: Unique identifier for the external source system
- `source_identifier`: Content identifier within the external system
- `source_url`: Direct URL to the content in the external system

This allows Bosca to integrate with and reference content from external systems while maintaining its own metadata layer.

## Security Model

### Permissions

Metadata implements a comprehensive permission system based on Bosca's security model:
- Permissions are assigned to Groups
- Principals (users) are assigned to Groups
- Metadata can have specific permissions for viewing, editing, managing, and deleting

This allows for fine-grained access control to metadata and its associated content.

### Public Access Permutations

Metadata supports different levels of public access through three boolean flags:

1. **`public`**: When true, allows unauthenticated access to metadata properties
2. **`public_content`**: When true, allows unauthenticated access to the primary content
3. **`public_supplementary`**: When true, allows unauthenticated access to supplementary content

These flags can be combined to create various access scenarios:
- Metadata only: Only basic information is publicly accessible
- Full content: Both metadata and primary content are publicly accessible
- Complete access: Metadata, primary content, and supplementary content are all publicly accessible

Public access is only granted when the metadata is in a `published` state, which is controlled through workflows.

## Content Types Built on Metadata

Bosca extends the Metadata model to support various specialized content types:

### Documents

[Documents](Documents.md) extend Metadata to allow for quickly building structured, user-facing content, such as
articles, videos, blog posts or other content designed for web pages or apps. Documents leverage content blocks
that can be organized flexibly to control the ordering and flow of content. These blocks can include media (
video/audio),
formatted text, images, and more.

### Guides

[Guides](Guides.md) extend Metadata to support building cyclical or repeatable content experiences. Guides can be
linear or calendar based. They leverage similar content blocks to documents and also include formats for creating
engaging, short-form visual content like shorts or stories. Guides are ideal for multi-day studies, plans, devotionals,
and daily story experiences.

### Supplementary Content

In addition to primary content, Metadata supports [Supplementary Content](Supplementary.md)—auxiliary content generated
by workflows.
For example, if Metadata refers to an audio or video file, a Workflow can create a transcription of the content, which
can then be stored as Supplementary Content within the Metadata system.

## Workflow Integration

Metadata serves as the foundation for [Workflows](Workflows.md) in Bosca. The Metadata model includes a `workflow` field
that helps make the state of the metadata more known.

Workflows can be triggered automatically based on content traits or manually initiated. They can also manage state
transitions for content, such as moving from draft to published status.

See the full schema: [Full Schema](GraphQL.md)
