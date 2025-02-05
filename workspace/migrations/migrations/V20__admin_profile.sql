insert into profiles (principal, name, visibility, created)
values (
    (select principal from principal_credentials where attributes ->>'identifier' = 'admin'),
        'Administrator',
    'public',
        now()
)