create type guide_type as enum ('linear', 'linear_progress', 'calendar', 'calendar_progress');
create type guide_step_module_type as enum ('text', 'richtext', 'video', 'audio', 'image', 'supplementary');
create type guide_metadata_attribute_type as enum ('string', 'int', 'float', 'date', 'datetime');

create table guide_templates
(
    id           bigserial  not null,
    name         varchar    not null,
    description  varchar    not null,
    rrule        varchar,
    type         guide_type not null,
    created      timestamp with time zone default now(),
    modified     timestamp with time zone default now(),
    has_progress boolean    not null      default false,
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

create table guide_template_steps
(
    template_id bigint    not null,
    id          bigserial not null,
    name        varchar   not null,
    description varchar   not null,
    sort        int       not null,
    primary key (template_id, id),
    foreign key (template_id) references guide_templates (id) on delete cascade
);

create table guide_template_step_metadata_attributes
(
    template_id bigint                        not null,
    id          bigint                        not null,
    key         varchar                       not null,
    name        varchar                       not null,
    description varchar                       not null,
    type        guide_metadata_attribute_type not null,
    primary key (template_id, id, key),
    foreign key (template_id, id) references guide_template_steps on delete cascade
);

create table guide_template_step_metadata_attribute_workflow_ids
(
    template_id bigint  not null,
    id          bigint  not null,
    key         varchar not null,
    workflow_id varchar not null,
    auto_run    bool    not null default false,
    primary key (template_id, id, key, workflow_id),
    foreign key (template_id, id, key) references guide_template_step_metadata_attributes (template_id, id, key) on delete cascade
);

create table guide_template_step_modules
(
    template_id      bigint                 not null,
    template_step_id bigint                 not null,
    id               bigserial              not null,
    type             guide_step_module_type not null,
    sort             int                    not null,
    text             varchar,
    richtext         jsonb,
    primary key (template_id, template_step_id, id)
);

create table guides
(
    metadata_id  uuid       not null,
    version      int        not null,
    rrule        varchar,
    type         guide_type not null,
    template_id  bigint,
    title        varchar    not null,
    has_progress boolean    not null default false,
    primary key (metadata_id, version),
    foreign key (metadata_id) references metadata (id) on delete cascade,
    foreign key (template_id) references guide_templates (id)
);

create table guide_steps
(
    metadata_id      uuid      not null,
    version          int       not null,
    template_id      bigint,
    template_step_id bigint,
    id               bigserial not null,
    step_metadata_id uuid      not null,
    sort             int       not null,
    text             varchar,
    richtext         jsonb,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references guides (metadata_id, version) on delete cascade,
    foreign key (step_metadata_id) references metadata (id),
    foreign key (template_id, template_step_id) references guide_template_steps (template_id, id)
);

create table guide_step_modules
(
    metadata_id        uuid                   not null,
    version            int                    not null,
    template_id        bigint,
    template_step_id   bigint,
    template_module_id bigint,
    id                 bigserial              not null,
    type               guide_step_module_type not null,
    sort               int                    not null,
    text               varchar,
    richtext           jsonb,
    primary key (metadata_id, version, id),
    foreign key (metadata_id, version) references guides (metadata_id, version) on delete cascade,
    foreign key (template_id, template_step_id, template_module_id) references guide_template_step_modules (template_id, template_step_id, id)
);

create table profile_guide_progress
(
    profile_id         uuid     not null,
    metadata_id        uuid     not null,
    version            int      not null,
    completed_step_ids bigint[] not null        default '{}',
    attributes         jsonb,
    started            timestamp with time zone default now(),
    modified           timestamp with time zone default now(),
    completed          timestamp with time zone,
    primary key (profile_id, metadata_id, version),
    foreign key (profile_id) references profiles (id),
    foreign key (metadata_id) references metadata (id)
);

create table profile_guide_history
(
    profile_id  uuid not null,
    metadata_id uuid not null,
    version     int  not null,
    attributes  jsonb,
    completed   timestamp with time zone,
    primary key (profile_id, metadata_id, version),
    foreign key (profile_id) references profiles (id),
    foreign key (metadata_id) references metadata (id)
);

create or replace function guide_history_trigger() returns trigger AS
$version_trigger$
begin
    if new.completed is not null then
        insert into profile_guide_history (profile_id, metadata_id, version, attributes, completed)
        values (new.profile_id, new.metadata_id, new.version, new.attributes, new.completed);
    end if;
    return new;
end;
$version_trigger$ language plpgsql;

create trigger guide_progress_history
    after update
    on profile_guide_history
    for each row
execute function guide_history_trigger();