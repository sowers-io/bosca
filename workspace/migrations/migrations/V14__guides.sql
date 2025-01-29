create type guide_type as enum ('linear', 'linear_progress', 'calendar', 'calendar_progress');
create type guide_module_type as enum ('text', 'richtext', 'video', 'audio', 'image', 'supplementary');
create type guide_metadata_attribute_type as enum ('string', 'int', 'float', 'date', 'datetime');

create table guide_templates
(
    id          bigserial not null,
    name        varchar   not null,
    description varchar   not null,
    created     timestamp with time zone default now(),
    modified    timestamp with time zone default now(),
    primary key (id)
);

create table guide_template_metadata_attributes
(
    id          bigint                        not null,
    key         varchar                       not null,
    name        varchar                       not null,
    description varchar                       not null,
    type        guide_metadata_attribute_type not null,
    primary key (id, key),
    foreign key (id) references guide_templates on delete cascade
);

create table guide_template_metadata_attribute_workflow_ids
(
    id          bigint  not null,
    key         varchar not null,
    workflow_id varchar not null,
    auto_run    bool    not null default false,
    primary key (id, key, workflow_id),
    foreign key (id, key) references guide_template_metadata_attributes (id, key) on delete cascade
);

create table guide_template_units
(
    template_id bigint    not null,
    id          bigserial not null,
    name        varchar   not null,
    description varchar   not null,
    sort        int       not null,
    primary key (template_id, id),
    foreign key (template_id) references guide_templates (id) on delete cascade
);

create table guide_template_unit_metadata_attributes
(
    template_id bigint                        not null,
    id          bigint                        not null,
    key         varchar                       not null,
    name        varchar                       not null,
    description varchar                       not null,
    type        guide_metadata_attribute_type not null,
    primary key (template_id, id, key),
    foreign key (template_id, id) references guide_template_units on delete cascade
);

create table guide_template_unit_modules
(
    template_id      bigint            not null,
    template_unit_id bigint            not null,
    id               bigserial         not null,
    type             guide_module_type not null,
    sort             int               not null,
    text             varchar,
    richtext         jsonb,
    primary key (template_id, template_unit_id, id),

);


create table guides
(
    metadata_id uuid    not null,
    version     int     not null,
    template_id bigint,
    title       varchar not null,
    primary key (metadata_id, version),
    foreign key (metadata_id) references metadata (id) on delete cascade,
    foreign key (template_id) references guide_templates (id)
);

create table guide_units
(
    metadata_id      uuid      not null,
    version          int       not null,
    template_id      bigint,
    template_unit_id bigint,
    id               bigserial not null,
    unit_metadata_id uuid      not null,
    sort             int       not null,
    text             varchar,
    richtext         jsonb,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references guides (metadata_id, version) on delete cascade,
    foreign key (unit_metadata_id) references metadata (id),
    foreign key (template_id, template_unit_id) references guide_template_units (template_id, id)
);

create table document_unit_modules
(
    metadata_id      uuid              not null,
    version          int               not null,
    template_id      bigint,
    template_unit_id bigint,
    id               bigserial         not null,
    type             guide_module_type not null,
    sort             int               not null,
    text             varchar,
    richtext         jsonb,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references guides (metadata_id, version) on delete cascade,
    foreign key (template_id, template_unit_id) references guide_template_units (template_id, id)
);
