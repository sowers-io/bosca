# Profiles
<primary-label ref="bosca"/>
<secondary-label ref="alpha"/>

Profiles are essential for delivering maximum value to both your users and your content. The more the system can learn
about a user, the better it can personalize their experience, prepare engaging content, and drive meaningful interactions.

**Bosca Profiles** are structured as a top-level profile with multiple attributes that provide further details about the
user. These attributes can originate directly from the user (e.g., their date of birth) or be derived from system observations
of user behavior.

## Key Features
- **Incremental Development**: There’s no need to burden users with lengthy onboarding flows. Instead, allow them to engage
  with the system naturally, offering opportunities to gradually share information about their preferences, interests, and dislikes.
- **Implicit Insights**: Through [Bosca Analytics](Analytics.md), user feedback and system observations are processed to
  generate implicit attributes. For instance, if a user frequently enjoys specific types of content, the system can generate
  recommendations or curated collections aligned with their preferences.

By combining explicit user inputs with implicit system-derived data, **Bosca Profiles** evolve dynamically, enabling
tailored experiences seamlessly via the AI/ML enabled workflows that can interact with the data as it flows through
the system.

## Data Model

```graphql
type Profile {
    attributes: [ProfileAttribute!]!
    id: String!
    name: String!
    slug: String
    visibility: ProfileVisibility!
}

type ProfileAttribute {
    attributes: JSON
    confidence: Int!
    expires: DateTime
    id: String!
    metadata: Metadata
    priority: Int!
    source: String!
    typeId: String!
    visibility: ProfileVisibility!
}

type ProfileAttributeType {
    description: String!
    id: String!
    name: String!
    visibility: ProfileVisibility!
}

type ProfileAttributeTypes {
    all: [ProfileAttributeType!]!
}
```

## Profile Bookmarks
<secondary-label ref="wip"/>
<secondary-label ref="concept"/>

```sql
create table profile_bookmarks
(
  id               bigserial not null,
  profile_id       uuid not null,
  metadata_id      uuid,
  metadata_version int,
  collection_id    uuid,
  created          timestamp with time zone default now(),
  primary key (id),
  foreign key (profile_id) references profiles (id) on delete cascade,
  foreign key (metadata_id) references metadata (id) on delete cascade,
  foreign key (collection_id) references collections (id) on delete cascade,
  check ( (metadata_id is not null and metadata_version is not null) or collection_id is not null ),
  unique (profile_id, metadata_id, metadata_version, collection_id)
);
```

## Profile Ratings
<secondary-label ref="wip"/>
<secondary-label ref="concept"/>

```sql
create table profile_ratings
(
  id               bigserial not null,
  profile_id       uuid not null,
  metadata_id      uuid not null,
  metadata_version int,
  collection_id    uuid,
  created          timestamp with time zone default now(),
  rating           int  not null,
  primary key (id),
  foreign key (collection_id) references collections (id) on delete cascade,
  foreign key (profile_id) references profiles (id) on delete cascade,
  check ( (metadata_id is not null and metadata_version is not null) or collection_id is not null ),
  foreign key (metadata_id) references metadata (id) on delete cascade,
  unique (profile_id, metadata_id, metadata_version, collection_id)
);
```

## Profile Guide Progress / History
<secondary-label ref="wip"/>
<secondary-label ref="concept"/>

```
create table profile_guide_progress
(
    profile_id         uuid     not null,
    metadata_id        uuid     not null,
    version            int      not null,
    attributes         jsonb,
    started            timestamp with time zone default now(),
    modified           timestamp with time zone default now(),
    completed          timestamp with time zone,
    primary key (profile_id, metadata_id, version),
    foreign key (profile_id) references profiles (id),
    foreign key (metadata_id) references metadata (id)
);

create table profile_guide_progress_steps
(
    profile_id  uuid   not null,
    metadata_id uuid   not null,
    version     int    not null,
    step_id     bigint not null,
    primary key (profile_id, metadata_id, version, step_id),
    foreign key (profile_id, metadata_id, version) references profile_guide_progress (profile_id, metadata_id, version) on delete cascade,
    foreign key (metadata_id, version, step_id) references guide_steps (metadata_id, version, id)
);

create table profile_guide_history
(
    id          bigserial not null,
    profile_id  uuid      not null,
    metadata_id uuid      not null,
    version     int       not null,
    attributes  jsonb,
    completed   timestamp with time zone,
    primary key (id),
    foreign key (profile_id) references profiles (id),
    foreign key (metadata_id) references metadata (id)
);
```