create extension if not exists unaccent;

create or replace function slugify(text)
    returns text AS
$$
declare
    value text := $1;
    slug  text := '';
    i     integer;
    c     text;
begin
    value := lower(value);
    value := unaccent(value);

    for i in 1..length(value)
        loop
            c := substring(value from i for 1);
            if c ~* '[a-z0-9]' then
                slug := slug || c;
            elsif c = ' ' or c = '-' then
                slug := slug || '-';
            else
                slug := slug || '-';
            end if;
        end loop;
    slug := trim(both '-' from slug);
    while slug like '%--%'
        loop
            slug := replace(slug, '--', '-');
        end loop;

    if slug = '' then
        slug := 'n-a';
    end if;

    return slug;
end;
$$ language plpgsql immutable;

create table slugs
(
    slug          varchar not null,
    metadata_id   uuid,
    collection_id uuid,
    primary key (slug),
    foreign key (metadata_id) references metadata (id) on delete cascade,
    foreign key (collection_id) references collections (id) on delete cascade
);

create sequence duplicate_slug_seq;

create or replace function metadata_slug_trigger() returns trigger AS
$trigger$
declare
    new_slug text := '';
    found    int;
begin
    new_slug := slugify(new.name);
    found := (select count(*) from slugs as s where s.slug = new_slug);
    if found > 0 then
        new_slug = new_slug || '-' || nextval('duplicate_slug_seq');
    end if;
    insert into slugs (slug, metadata_id) values (new_slug, new.id);
    return new;
end;
$trigger$
    language plpgsql;

create trigger metadata_slug
    after insert
    on metadata
    for each row
execute function metadata_slug_trigger();

create or replace function collection_slug_trigger() returns trigger AS
$trigger$
declare
    new_slug text := '';
    found    int;
begin
    if new.type in ('root', 'system', 'queue') then
        return new;
    end if;
    new_slug := slugify(new.name);
    found := (select count(*) from slugs as s where s.slug = new_slug);
    if found > 0 then
        new_slug = new_slug || '-' || nextval('duplicate_slug_seq');
    end if;
    insert into slugs (slug, collection_id) values (new_slug, new.id);
    return new;
end;
$trigger$
    language plpgsql;

create trigger collection_slug
    after insert
    on collections
    for each row
execute function collection_slug_trigger();