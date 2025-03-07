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
    val templateType: String? = null
)

class CollectionsInstaller(val client: Client) : Installer {

    data class PendingInstall(
        val collection: CollectionInput,
        val collectionTemplate: CollectionTemplateInput?,
        val documentTemplate: DocumentTemplateInput?
    )

    private fun CollectionDefinition.toPendingInstall(categories: Map<String, Category>): PendingInstall {
        return PendingInstall(
            toInput(categories),
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
        if (slug?.collection?.id == null) {
            client.collections.add(collection.toInput(currentCategories))
        } else {
            client.collections.edit(slug.collection!!.id, install.collection)
        }
        collection.templates?.collection?.let {
            val templateSlug = client.get(collection.slug + "-collection-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            if (templateSlug?.metadata?.id == null) {
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
            }
        }
        collection.templates?.document?.let {
            val templateSlug = client.get(collection.slug + "-document-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            if (templateSlug?.metadata?.id == null) {
                client.metadata.add(
                    collection.templates.document.toDocumentTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            } else if (templateSlug.metadata?.workflow?.metadataWorkflow?.state != "published") {
                client.metadata.edit(
                    templateSlug.metadata!!.id,
                    collection.templates.document.toDocumentTemplateInput(
                        templatesId,
                        collection,
                        currentCategories
                    )
                )
            }
        }
        collection.templates?.guide?.let {
            val templateSlug = client.get(collection.slug + "-guide-template")
            if (templateSlug?.metadata?.ready != null && templateSlug.metadata?.workflow?.metadataWorkflow?.state == "published") {
                return@let
            }
            if (templateSlug?.metadata?.id == null) {
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
