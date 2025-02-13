drop table document_block_metadata_references;
drop table document_blocks;
drop table documents;
drop table document_template_blocks;
drop table document_template_metadata_attribute_workflow_ids;
drop table document_template_metadata_attributes;
drop table document_templates;

drop type document_block_type;
drop type document_metadata_attribute_type;

create type document_block_type as enum ('text', 'richtext', 'video', 'audio', 'image', 'supplementary');
create type document_attribute_type as enum ('string', 'int', 'float', 'date', 'datetime');

create table document_templates
(
    metadata_id               uuid    not null,
    version                   int     not null,
    allow_user_defined_blocks boolean not null default true,
    primary key (metadata_id, version),
    foreign key (metadata_id) references metadata (id) on delete cascade
);

create table document_template_categories
(
    metadata_id uuid not null,
    version     int  not null,
    category_id uuid not null,
    primary key (metadata_id, version, category_id),
    foreign key (metadata_id, version) references document_templates (metadata_id, version) on delete cascade,
    foreign key (category_id) references categories (id) on delete cascade
);

create table document_template_attributes
(
    metadata_id uuid                    not null,
    version     int                     not null,
    key         varchar                 not null,
    name        varchar                 not null,
    description varchar                 not null,
    type        document_attribute_type not null,
    primary key (metadata_id, version, key),
    foreign key (metadata_id, version) references document_templates (metadata_id, version) on delete cascade
);

create table document_template_attribute_workflow_ids
(
    metadata_id uuid    not null,
    version     int     not null,
    key         varchar not null,
    workflow_id varchar not null,
    auto_run    bool    not null default false,
    primary key (metadata_id, version, key, workflow_id),
    foreign key (metadata_id, version, key) references document_template_attributes (metadata_id, version, key) on delete cascade,
    foreign key (workflow_id) references workflows (id)
);

create table document_template_blocks
(
    metadata_id uuid                not null,
    version     int                 not null,
    id          bigserial           not null,
    sort        int                 not null,
    name        varchar             not null,
    description varchar             not null,
    type        document_block_type not null,
    validation  jsonb,
    content     jsonb               not null,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references document_templates (metadata_id, version) on delete cascade
);

create table documents
(
    metadata_id               uuid    not null,
    version                   int     not null,
    template_metadata_id      uuid,
    template_metadata_version int,
    title                     varchar not null,
    allow_user_defined_blocks boolean not null default true,
    primary key (metadata_id, version),
    foreign key (metadata_id) references metadata (id) on delete cascade,
    foreign key (template_metadata_id, template_metadata_version) references document_templates (metadata_id, version)
);

create table document_blocks
(
    metadata_id uuid                not null,
    version     int                 not null,
    id          bigserial           not null,
    type        document_block_type not null,
    sort        int                 not null,
    content     jsonb               not null,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references documents (metadata_id, version) on delete cascade
);

create table document_block_metadata
(
    metadata_id           uuid   not null,
    version               int    not null,
    block_id              bigint not null,
    metadata_reference_id uuid   not null,
    attributes            jsonb,
    sort                  int    not null,
    primary key (metadata_id, version, block_id, metadata_reference_id),
    foreign key (metadata_id, version, block_id) references document_blocks (metadata_id, version, id) on delete cascade,
    foreign key (metadata_reference_id) references metadata (id) on delete cascade
);