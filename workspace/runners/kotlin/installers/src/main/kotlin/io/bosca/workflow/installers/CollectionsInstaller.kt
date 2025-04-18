package io.bosca.workflow.installers

import io.bosca.api.Client
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.*
import io.bosca.installer.Installer
import io.bosca.util.toOptional
import io.bosca.workflow.ext.toCollectionTemplateInput
import io.bosca.workflow.ext.toDocumentTemplateInput
import io.bosca.workflow.ext.toGuideTemplateInput
import io.bosca.workflow.ext.toInput
import io.bosca.workflow.models.CollectionDefinition
import io.bosca.workflow.models.Collections
import io.bosca.workflow.yaml.YamlLoader
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import java.io.File

@Serializable
data class EditorConfiguration(
    @SerialName("editor.type")
    val editorType: String,
    @SerialName("template.type")
    val templateType: String? = null,
    @SerialName("template.type.sub")
    val subTemplateType: String? = null
)

class CollectionsInstaller(val client: Client) : Installer {

    data class PendingInstall(
        val collection: CollectionInput,
        val collectionTemplate: CollectionTemplateInput?,
        val documentTemplate: DocumentTemplateInput?
    )

    private suspend fun CollectionDefinition.toPendingInstall(categories: Map<String, Category>): PendingInstall {
        return PendingInstall(
            toInput(client, categories),
            templates?.collection?.toInput(),
            templates?.document?.toInput(),
        )
    }

    private suspend fun install(
        client: Client,
        collection: CollectionDefinition,
        currentCategories: Map<String, Category>,
        templatesId: String
    ) {
        val install = collection.toPendingInstall(currentCategories)
        val slug = client.get(collection.slug)
        val id = if (slug?.collection?.id == null) {
            client.collections.add(install.collection)
        } else {
            client.collections.edit(slug.collection!!.id, install.collection)
        } ?: error("Collection not found")
        val groups = client.security.getGroups(0, 100).associateBy { it.name }
        collection.permissions?.let {
            for (permission in it) {
                client.collections.addPermission(
                    PermissionInput(
                        action = PermissionAction.valueOf(permission.action.uppercase()),
                        entityId = id,
                        groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                    )
                )
            }
        }
        collection.templates?.collection?.let {
            val templateSlug = client.get(collection.slug + "-collection-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            val metadataId = if (templateSlug?.metadata?.id == null) {
                client.metadata.add(
                    collection.templates.collection.toCollectionTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else if (templateSlug.metadata?.workflow?.metadataWorkflow?.state != "published") {
                client.metadata.edit(
                    templateSlug.metadata!!.id,
                    collection.templates.collection.toCollectionTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else {
                templateSlug.metadata?.id
            }
            metadataId?.let { mid ->
                it.permissions?.let {
                    for (permission in it) {
                        client.metadata.addPermission(
                            PermissionInput(
                                action = PermissionAction.valueOf(permission.action.uppercase()),
                                entityId = mid,
                                groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                            )
                        )
                    }
                }
            }
        }
        collection.templates?.document?.let {
            val templateSlug = client.get(collection.slug + "-document-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            val metadataId = if (templateSlug?.metadata?.id == null) {
                client.metadata.add(
                    collection.templates.document.toDocumentTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else if (templateSlug.metadata?.workflow?.metadataWorkflow?.state != "published" &&
                templateSlug.metadata?.workflow?.metadataWorkflow?.state != "pending") {
                client.metadata.edit(
                    templateSlug.metadata!!.id,
                    collection.templates.document.toDocumentTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else {
                templateSlug.metadata?.id
            }
            metadataId?.let { mid ->
                it.permissions?.let {
                    for (permission in it) {
                        client.metadata.addPermission(
                            PermissionInput(
                                action = PermissionAction.valueOf(permission.action.uppercase()),
                                entityId = mid,
                                groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                            )
                        )
                    }
                }
            }
        }
        collection.templates?.guide?.let {
            val templateSlug = client.get(collection.slug + "-guide-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            val metadataId = if (templateSlug?.metadata?.id == null) {
                client.metadata.add(
                    collection.templates.guide.toGuideTemplateInput(
                        client,
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else if (templateSlug.metadata?.workflow?.metadataWorkflow?.state != "published") {
                client.metadata.edit(
                    templateSlug.metadata!!.id,
                    collection.templates.guide.toGuideTemplateInput(
                        client,
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else {
                templateSlug.metadata?.id
            }
            metadataId?.let { mid ->
                it.permissions?.let {
                    for (permission in it) {
                        client.metadata.addPermission(
                            PermissionInput(
                                action = PermissionAction.valueOf(permission.action.uppercase()),
                                entityId = mid,
                                groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                            )
                        )
                    }
                    val guide = client.metadata.getGuideTemplate(mid, templateSlug?.metadata?.version ?: 1)
                    for (step in guide?.steps ?: emptyList()) {
                        for (permission in it) {
                            client.metadata.addPermission(
                                PermissionInput(
                                    action = PermissionAction.valueOf(permission.action.uppercase()),
                                    entityId = step.guideTemplateStep.metadata?.metadata?.id ?: continue,
                                    groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                                )
                            )
                        }
                        for (module in step.guideTemplateStep.modules) {
                            for (permission in it) {
                                client.metadata.addPermission(
                                    PermissionInput(
                                        action = PermissionAction.valueOf(permission.action.uppercase()),
                                        entityId = module.guideTemplateStepModule.metadata?.metadata?.id ?: continue,
                                        groupId = groups[permission.group]?.id ?: error("Group not found: ${permission.group}")
                                    )
                                )
                            }
                        }
                    }
                }
            }
        }
        val collections = collection.collections
        if (collections != null) {
            for (child in collections) {
                install(client, child, currentCategories, templatesId)
            }
        }
    }

    override suspend fun install(client: Client, directory: File) {
        val collections = YamlLoader.load(
            Collections.serializer(),
            File(directory, "templates"),
            File(directory, "templates/collections.yaml")
        )
        val categories = client.categories.getAll().associateBy { it.name }
        val templates = client.get("templates")
        val templatesId = if (templates?.collection?.id == null) {
            client.collections.add(
                CollectionInput(
                    slug = "templates".toOptional(),
                    name = "Templates",
                    collectionType = CollectionType.SYSTEM.toOptional(),
                )
            )
        } else {
            templates.collection?.id
        } ?: error("Templates collection not found")
        for (collection in collections.collections) {
            install(client, collection, categories, templatesId)
        }
    }
}
