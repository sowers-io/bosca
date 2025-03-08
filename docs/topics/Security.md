# Security
<primary-label ref="bosca"/>
<secondary-label ref="alpha"/>

Bosca’s security model is built on four key components: Principals, Credentials (e.g., passwords, Google Auth, Facebook
Auth—third-party authorization coming soon), Groups, and Permissions.

It supports both Authorization (via Bearer tokens), Basic Authentication (via username/password), or Cookie (via JWT).
When using Bearer tokens, Bosca generates a JWT token to handle authorization. Future updates will also incorporate
support for refresh tokens.

Groups play a central role in defining permissions, and Principals are assigned to these groups. Both Metadata and
Collections come with permission settings, enabling fine-grained access control.

Depending on the assigned group and its permissions, Principals can perform a variety of actions. For example:

- **Collections**: Principals can list, view, edit, or delete items.
- **Workflows**: Principals can execute workflows.
- **Metadata**: Principals can view, edit, manage, or delete metadata.

This flexible security model ensures precise control over user access and actions across Bosca’s features.

In addition to permissions, [Collections](Collections.md) can be marked as `public` or `publicList`.  Allowing
unauthenticated users to
access the content when the Collection is in a `published` state (see [Workflows](Workflows.md) for more information).

In addition to permissions, [Metadata](Metadata.md) can be marked as `public`, `publicContent`, or
`publicSupplementary`. Allowing unauthenticated users to access the content when the Metadata is in a `published`
state (see [Workflows](Workflows.md) for more information).

## Principal

```graphql
type Principal {
    groups: [Group!]!
    id: String!
}

type Group {
    id: String!
    name: String!
}
```

## Permissions

```graphql
enum PermissionAction {
    DELETE
    EDIT
    EXECUTE
    LIST
    MANAGE
    VIEW
}

type Permission {
    action: PermissionAction!
    group: Group!
    groupId: String!
}
```

[See More](https://github.com/sowers-io/bosca/tree/main/workspace/server/src/security)