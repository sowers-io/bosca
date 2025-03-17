package io.bosca.workflow.installers

import com.apollographql.apollo.api.toUpload
import io.bosca.api.Client
import io.bosca.graphql.type.*
import io.bosca.installer.Installer
import io.bosca.util.toOptional
import io.bosca.workflow.models.AssetDefinition
import io.bosca.workflow.models.AssetFolder
import io.bosca.workflow.models.AssetsDefinition
import io.bosca.workflow.yaml.YamlLoader
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
    val file = File(directory, file)
    val mimeType = mimeType ?: withContext(Dispatchers.IO) {
        Files.probeContentType(file.toPath())
    } ?: "application/octet-stream"
    val metadata = MetadataInput(
        name = file.name,
        attributes = mapOf(
            "description" to description,
            "original.name" to file.name
        ).toOptional(),
        contentType = mimeType,
        languageTag = "en",
        metadataType = MetadataType.STANDARD.toOptional(),
        parentCollectionId = parentCollectionId.toOptional(),
        slug = file.name.split(".").first().toOptional(),
    )
    val current = client.get(metadata.slug.getOrThrow()!!)
    val id = if (current == null) {
        client.metadata.add(metadata)
    } else {
        current.metadata?.id
    } ?: error("failed to add metadata for file ${file.name}")
    client.metadata.setFileContents(id, file.toUpload(mimeType))
    client.metadata.setPublic(id, public)
    client.metadata.setPublicContent(id, publicContent)
    if (ready) {
        client.metadata.setReady(id)
    }
    if (publish) {
        while (true) {
            val m = client.metadata.get(id)
            val status = m?.workflow?.metadataWorkflow ?: break
            if (status.state == "pending") {
                delay(10)
            } else if (status.state == "failed") {
                break
            } else if (status.state == "draft") {
                client.workflows.beginMetadataTransition(
                    m.id,
                    m.version,
                    "published",
                    "publishing asset from installer"
                )
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