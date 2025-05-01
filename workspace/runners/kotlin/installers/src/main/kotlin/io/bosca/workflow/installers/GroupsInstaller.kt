package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.graphql.type.PermissionAction
import io.bosca.graphql.type.PermissionInput
import io.bosca.installer.Installer
import io.bosca.workflow.models.Groups
import io.bosca.yaml.YamlLoader
import java.io.File

class GroupsInstaller: Installer {

    override suspend fun install(client: Client, directory: File) {
        val current = client.security.getGroups(0, 100000).mapTo(mutableSetOf()) { it.name }
        val groups = YamlLoader.load(Groups.serializer(), directory, File(directory, "groups.yaml"))
        for (group in groups.groups) {
            if (!current.contains(group.name)) {
                client.security.addGroup(group.name, group.description)
            }
            val id = client.security.getGroups(0, 1000).first { it.name == group.name }.id
            if (group.permissions != null) {
                for (permission in group.permissions) {
                    val content = client.get(permission.slug)
                    client.collections.addPermission(PermissionInput(
                        action = PermissionAction.valueOf(permission.action.uppercase()),
                        groupId = id,
                        entityId = content?.collection?.id ?: content?.metadata?.id ?: error("missing content for: ${permission.slug}")
                    ))
                }
            }
        }
    }
}
