package io.bosca.workflow

import com.apollographql.apollo.api.toUpload
import io.bosca.api.Client
import io.bosca.graphql.fragment.*
import io.bosca.graphql.type.CollectionSupplementaryInput
import io.bosca.graphql.type.MetadataSupplementaryInput
import io.bosca.util.*
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.SerializationStrategy
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.JsonElement
import kotlinx.serialization.json.decodeFromJsonElement
import kotlinx.serialization.json.encodeToJsonElement
import java.io.File
import io.bosca.graphql.type.ActivityInput as ActivityDefintion

class ActivityContext {
    private val files = mutableListOf<File>()

    fun addFile(file: File) {
        files.add(file)
    }

    suspend fun newTemporaryFile(job: WorkflowJob, extension: String): File {
        val file = withContext(Dispatchers.IO) {
            File.createTempFile(job.id.id, ".$extension")
        }
        addFile(file)
        return file
    }

    fun cleanup() {
        files.forEach { it.deleteRecursively() }
    }
}

abstract class Activity(protected val client: Client) {

    abstract val id: String

    abstract suspend fun toActivityDefinition(): ActivityDefintion

    protected fun getInputParameter(
        job: WorkflowJob,
        parameter: String
    ): WorkflowActivityParameter? {
        return job.workflowActivity
            .workflowActivity
            .inputs
            .firstOrNull { it.workflowActivityParameter.name == parameter }
            ?.workflowActivityParameter
    }

    protected fun getOutputParameter(
        job: WorkflowJob,
        parameter: String,
    ): WorkflowActivityParameter? {
        return job.workflowActivity
            .workflowActivity
            .outputs
            .firstOrNull { it.workflowActivityParameter.name == parameter }
            ?.workflowActivityParameter
    }

    protected fun getInputParameterValue(job: WorkflowJob, parameter: String): String {
        val p = getInputParameter(job, parameter)
        return p?.value ?: error("missing parameter: ${job.planId} :: ${job.id} :: $parameter")
    }

    protected suspend fun deleteMetadataSupplementary(job: WorkflowJob, identifier: String) {
        val parameter = getInputParameter(job, identifier)
            ?: error("missing parameter: ${job.planId} :: ${job.id} :: $identifier")
        val supplementary = job.getMetadataSupplementary(parameter)
        if (supplementary == null) error("missing supplementary: ${job.planId} :: ${job.id} :: $identifier")
        client.metadata.deleteSupplementary(supplementary.id)
    }

    protected suspend fun deleteCollectionSupplementary(job: WorkflowJob, identifier: String) {
        val parameter = getInputParameter(job, identifier)
            ?: error("missing parameter: ${job.planId} :: ${job.id} :: $identifier")
        val supplementary = job.getCollectionSupplementary(parameter)
        if (supplementary == null) error("missing supplementary: ${job.planId} :: ${job.id} :: $identifier")
        client.collections.deleteSupplementary(supplementary.id)
    }

    protected suspend fun getContentFile(context: ActivityContext, job: WorkflowJob, metadata: Metadata): File {
        val content = client.metadata.getMetadataContentDownload(metadata.id)
            ?: error("missing content")
        val file = context.newTemporaryFile(job, content.type.split("/").last())
        client.files.download(content.urls.download, file)
        return file
    }

    protected suspend fun setContent(job: WorkflowJob, file: File) {
        val upload = client.metadata.getMetadataContentUpload(
            job.metadata?.metadata?.id ?: error("missing metadata id")
        ) ?: error("missing upload")
        client.files.upload(upload, file)
    }

    protected suspend fun getContentFile(context: ActivityContext, job: WorkflowJob): File =
        getContentFile(context, job, job.metadata?.metadata ?: error("missing metadata"))

    protected suspend fun downloadToFile(context: ActivityContext, job: WorkflowJob, url: String): File {
        val file = withContext(Dispatchers.IO) {
            val extension = (job.metadata?.metadata?.content?.metadataContent?.type?.split("/")?.last() ?: ".download")
            File.createTempFile(job.id.id, ".$extension")
        }
        context.addFile(file)
        client.files.download(url, file)
        return file
    }

    protected fun hasInputs(job: WorkflowJob): Boolean =
        job.workflowActivity.workflowActivity.inputs.isNotEmpty()

    protected fun hasOutputs(job: WorkflowJob): Boolean =
        job.workflowActivity.workflowActivity.outputs.isNotEmpty()

    protected fun hasSupplementary(job: WorkflowJob, key: String): Boolean {
        val supplementaryKey = getInputParameter(job, key) ?: return false
        val supplementary = job.metadata?.metadata?.supplementary?.firstOrNull {
            it.metadataSupplementary.key == supplementaryKey.value
        }?.metadataSupplementary
        return supplementary != null
    }

    protected suspend fun getInputSupplementaryFile(context: ActivityContext, job: WorkflowJob, identifier: String): File {
        val parameter = getInputParameter(job, identifier) ?: error("missing supplementary key: $identifier")
        return getInputSupplementaryFile(context, job, parameter)
    }

    protected suspend fun getInputSupplementaryFile(context: ActivityContext, job: WorkflowJob, parameter: WorkflowActivityParameter): File {
        val identifier = parameter.name
        job.metadata?.metadata?.let {
            val supplementary = job.getMetadataSupplementary(parameter) ?: error("missing supplementary: ${job.planId.id} -> $identifier")
            val download = client.metadata.getSupplementaryContentDownload(supplementary.id)
                ?: error("missing supplementary: ${job.planId.id} -> $identifier -> $identifier")
            val file = context.newTemporaryFile(job, download.type.split("/").last())
            client.files.download(download.urls.download, file)
            return file
        }
        job.collection?.collection?.let {
            val supplementary = job.getCollectionSupplementary(parameter) ?: error("missing supplementary: ${job.planId.id} -> $identifier")
            val download = client.collections.getSupplementaryContentDownload(supplementary.id)
                ?: error("missing supplementary: ${job.planId.id} -> $identifier -> $identifier")
            val file = context.newTemporaryFile(job, download.type.split("/").last())
            client.files.download(download.urls.download, file)
            return file
        }
        error("missing collection or metadata: $identifier")
    }

    protected suspend fun getInputSupplementaryText(
        context: ActivityContext,
        job: WorkflowJob,
        key: String
    ): String {
        val file = getInputSupplementaryFile(context, job, key)
        return file.readText()
    }

    protected suspend inline fun <reified T> decodeInputSupplementary(
        context: ActivityContext,
        job: WorkflowJob,
        key: String
    ): T {
        val text = getInputSupplementaryText(context, job, key)
        return Json.decodeFromString(text)
    }

    protected suspend fun setContents(
        job: WorkflowJob,
        value: String,
        contentType: String = job.metadata?.metadata?.content?.metadataContent?.type ?: "text/plain"
    ) {
        val metadataId = job.metadata?.metadata?.id ?: error("missing metadata id")
        client.metadata.setTextContent(
            metadataId,
            contentType,
            value
        )
    }

    protected suspend fun getOrAddMetadataSupplementary(
        job: WorkflowJob,
        parameter: String,
        name: String,
        contentType: String,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ): MetadataSupplementary {
        val output = getOutputParameter(job, parameter) ?: error("missing supplementary (${job.workflowActivity.workflowActivity.activityId}): $parameter")
        return job.metadata?.metadata?.supplementary?.firstOrNull {
            it.metadataSupplementary.key == output.value && (it.metadataSupplementary.planId == job.planId.id || it.metadataSupplementary.planId == null)
        }?.metadataSupplementary
            ?: client.metadata.addSupplementary(
                MetadataSupplementaryInput(
                    planId = job.planId.id,
                    name = name,
                    contentType = contentType,
                    key = output.value,
                    metadataId = job.metadata?.metadata?.id ?: error("missing metadata id"),
                    sourceId = sourceId.toOptional(),
                    sourceIdentifier = sourceIdentifier.toOptional()
                )
            ) ?: error("missing supplementary: $parameter")
    }

    protected suspend fun getOrAddCollectionSupplementary(
        job: WorkflowJob,
        parameter: String,
        name: String,
        contentType: String,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ): CollectionSupplementary {
        val output = getOutputParameter(job, parameter) ?: error("missing supplementary: $parameter")
        return job.collection?.collection?.supplementary?.firstOrNull {
            it.collectionSupplementary.key == output.value && (it.collectionSupplementary.planId == job.planId.id || it.collectionSupplementary.planId == null)
        }?.collectionSupplementary
            ?: client.collections.addSupplementary(
                CollectionSupplementaryInput(
                    planId = job.planId.id,
                    name = name,
                    contentType = contentType,
                    key = output.value,
                    collectionId = job.collection?.collection?.id ?: error("missing collection id"),
                    sourceId = sourceId.toOptional(),
                    sourceIdentifier = sourceIdentifier.toOptional()
                )
            ) ?: error("missing supplementary: $parameter")
    }

    protected suspend fun setSupplementaryContents(
        job: WorkflowJob,
        parameter: String,
        name: String,
        content: String,
        contentType: String,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ) {
        job.metadata?.metadata?.let {
            val supplementary = getOrAddMetadataSupplementary(job, parameter, name, contentType, sourceId, sourceIdentifier)
            client.metadata.setSupplementaryTextContent(
                supplementary.id,
                contentType,
                content
            )
        } ?: job.collection?.collection?.let {
            val supplementary =
                getOrAddCollectionSupplementary(job, parameter, name, contentType, sourceId, sourceIdentifier)
            client.collections.setSupplementaryTextContent(
                supplementary.id,
                contentType,
                content
            )
        } ?: error("missing metadata or collection")
    }

    protected suspend fun setSupplementaryContents(
        job: WorkflowJob,
        parameter: String,
        name: String,
        file: File,
        contentType: String,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ) {
        val supplementary = getOrAddMetadataSupplementary(job, parameter, name, contentType, sourceId, sourceIdentifier)
        client.metadata.setSupplementaryContents(
            supplementary.id,
            file.toUpload(contentType),
        )
    }

    protected suspend fun <T> setSupplementaryContents(
        job: WorkflowJob,
        output: String,
        name: String,
        serializer: SerializationStrategy<T>,
        value: T,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ) {
        job.metadata?.metadata?.let {
            val supplementary = getOrAddMetadataSupplementary(
                job,
                output,
                name,
                "application/json",
                sourceId,
                sourceIdentifier
            )
            client.metadata.setSupplementaryTextContent(
                supplementary.id,
                "application/json",
                if (value is String) value else Json.encodeToString(serializer, value),
            )
        } ?: job.collection?.collection?.let {
            val supplementary = getOrAddCollectionSupplementary(
                job,
                output,
                name,
                "application/json",
                sourceId,
                sourceIdentifier
            )
            client.collections.setSupplementaryTextContent(
                supplementary.id,
                "application/json",
                if (value is String) value else Json.encodeToString(serializer, value),
            )
        } ?: error("missing metadata or collection")
    }

    protected inline fun <reified T> getAttributes(job: WorkflowJob, name: String): T {
        @Suppress("UNCHECKED_CAST")
        val attributes =
            ((job.metadata?.metadata?.attributes ?: job.collection?.collection?.attributes) as Map<String, Any?>?)
                ?: emptyMap()

        @Suppress("UNCHECKED_CAST")
        val values = attributes[name] as? Map<String, Any?> ?: emptyMap()
        return Json.decodeFromJsonElement<T>(values.toJsonElement())
    }

    protected suspend inline fun <reified T> setAttribute(job: WorkflowJob, name: String, value: T) {
        val data = Json.encodeToJsonElement(value).toAny()
        if (job.metadata != null) {
            @Suppress("UNCHECKED_CAST")
            val attrs = job.metadata.metadata.attributes as MutableMap<String, Any?>? ?: mutableMapOf()
            attrs[name] = data
            client.metadata.setAttributes(job.metadata.metadata.id, attrs)
        } else if (job.collection != null) {
            @Suppress("UNCHECKED_CAST")
            val attrs = job.collection.collection.attributes as MutableMap<String, Any?>? ?: mutableMapOf()
            attrs[name] = data
            client.collections.setAttributes(job.collection.collection.id, attrs)
        } else {
            error("missing metadata or collection")
        }
    }

    protected suspend fun setAttribute(job: WorkflowJob, name: String, data: JsonElement) {
        if (job.metadata != null) {
            @Suppress("UNCHECKED_CAST")
            val attrs = job.metadata.metadata.attributes as MutableMap<String, Any?>? ?: mutableMapOf()
            attrs[name] = data.toAny()
            client.metadata.setAttributes(job.metadata.metadata.id, attrs)
        } else if (job.collection != null) {
            @Suppress("UNCHECKED_CAST")
            val attrs = job.collection.collection.attributes as MutableMap<String, Any?>? ?: mutableMapOf()
            attrs[name] = data.toAny()
            client.collections.setAttributes(job.collection.collection.id, attrs)
        } else {
            error("missing metadata or collection")
        }
    }

    protected suspend fun setAttribute(job: WorkflowJob, value: Any?) {
        if (job.metadata != null) {
            client.metadata.setAttributes(job.metadata.metadata.id, value)
        } else if (job.collection != null) {
            client.collections.setAttributes(job.collection.collection.id, value)
        } else {
            error("missing metadata or collection")
        }
    }

    protected inline fun <reified T> getContext(job: WorkflowJob): T {
        return Json.decodeFromJsonElement<T>((job.context ?: emptyMap<String, Any>()).toJsonElement())
    }

    protected suspend inline fun <reified T> setContext(job: WorkflowJob, value: T) {
        val data = Json.encodeToJsonElement(value).toAny()
        client.workflows.setWorkflowJobContext(job.id, data ?: emptyMap<String, Any>())
    }

    protected inline fun <reified T> getConfiguration(job: WorkflowJob): T {
        return Json.decodeFromJsonElement<T>((job.workflowActivity.workflowActivity.configuration).toJsonElement())
    }

    abstract suspend fun execute(context: ActivityContext, job: WorkflowJob)
}