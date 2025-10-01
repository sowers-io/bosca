package io.bosca.workflow.installers

import com.apollographql.apollo.api.toUpload
import io.bosca.api.Client
import io.bosca.graphql.type.*
import io.bosca.installer.Installer
import io.bosca.util.toOptional
import io.bosca.workflow.models.AssetDefinition
import io.bosca.workflow.models.AssetFolder
import io.bosca.workflow.models.AssetsDefinition
import io.bosca.yaml.YamlLoader
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.delay
import kotlinx.coroutines.withContext
import java.io.File
import java.nio.file.Files

suspend fun AssetFolder.install(client: Client, parentCollectionId: String, directory: File) {
    val assetsCollection = client.get(slug)
    val assetsCollectionId = if (assetsCollection?.collection == null) {
        val collection = CollectionInput(
            slug = slug.toOptional(),
            name = name,
            description = description.toOptional(),
            collectionType = CollectionType.FOLDER.toOptional(),
            parentCollectionId = parentCollectionId.toOptional()
        )
        val id = client.collections.add(collection) ?: error("failed to add assets collection")
        client.collections.setPublic(id, public)
        if (ready) {
            client.collections.setReady(id)
        }
        id
    } else {
        assetsCollection.collection!!.id
    }
    for (folder in folders) {
        folder.install(client, assetsCollectionId, directory)
    }
    for (asset in assets) {
        asset.install(client, assetsCollectionId, directory)
    }
}

suspend fun AssetDefinition.install(client: Client, parentCollectionId: String, directory: File) {
    val (file, mimeType) = file?.let {
        val file = File(directory, file)
        val mimeType = mimeType ?: withContext(Dispatchers.IO) {
            Files.probeContentType(file.toPath())
        } ?: "application/octet-stream"
        Pair(file, mimeType)
    } ?: Pair(null, null)
    val attributes = attributes as? Map<*, *> ?: emptyMap<String, Any>()
    val metadata = MetadataInput(
        name = name,
        attributes = (attributes + mapOf(
            "description" to description,
            "original.name" to file?.name
        )).toOptional(),
        contentType = mimeType ?: "application/octet-stream",
        languageTag = "en",
        metadataType = MetadataType.STANDARD.toOptional(),
        parentCollectionId = parentCollectionId.toOptional(),
        slug = slug.toOptional()
    )
    val current = client.get(metadata.slug.getOrThrow()!!)
    val id = if (current == null) {
        client.metadata.add(metadata)
    } else {
        if (current.metadata?.metadataWorkflow?.metadataWorkflow?.state == "draft") {
            client.metadata.edit(current.metadata?.id ?: error("failed to edit metadata for asset $name"), metadata)
        } else {
            current.metadata?.id
        }
    } ?: error("failed to add metadata for asset $name")
    file?.let {
        client.metadata.setFileContents(id, it.toUpload(mimeType!!))
    }
    client.metadata.setPublic(id, public)
    client.metadata.setPublicContent(id, publicContent)
    if (ready) {
        client.metadata.setReady(id)
    }
    if (publish) {
        while (true) {
            val m = client.metadata.get(id)
            val status = m?.metadataWorkflow?.metadataWorkflow ?: break
            if (status.state == "pending") {
                delay(10)
            } else if (status.state == "failed") {
                break
            } else if (status.state == "draft") {
                try {
                    if (m.metadataWorkflow?.metadataWorkflow?.pending != null) {
                        client.workflows.cancelMetadataTransition(m.id, m.version)
                    }
                    client.workflows.beginMetadataTransition(
                        m.id,
                        m.version,
                        "published",
                        "publishing asset from installer"
                    )
                } catch (e: Exception) {
                    println("Failed to publish asset $name: ${e.message}")
                    break
                }
            } else if (status.state == "published") {
                break
            }
        }
    }
}

class AssetsInstaller : Installer {

    override suspend fun install(client: Client, directory: File) {
        val definition = YamlLoader.load(AssetsDefinition.serializer(), directory, File(directory, "assets.yaml"))
        val assets = AssetFolder(
            slug = "assets",
            name = "Assets",
            description = "Assets",
            folders = definition.folders,
            assets = definition.assets
        )
        assets.install(client, "00000000-0000-0000-0000-000000000000", directory)
    }
}