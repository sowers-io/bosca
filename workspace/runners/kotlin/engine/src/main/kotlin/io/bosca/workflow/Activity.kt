package io.bosca.workflow

import com.apollographql.apollo.api.Optional
import io.bosca.api.Client
import io.bosca.graphql.fragment.Metadata
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.MetadataSupplementaryInput
import io.bosca.util.toAny
import io.bosca.util.toJsonElement
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

    fun cleanup() {
        files.forEach { it.deleteRecursively() }
    }
}

abstract class Activity(protected val client: Client) {

    abstract val id: String

    abstract suspend fun toActivityDefinition(): ActivityDefintion

    protected fun getInputParameterValue(job: WorkflowJob, name: String): String {
        val parameter = job.workflowActivity
            .workflowActivity
            .inputs
            .firstOrNull { it.workflowActivityParameter.name == name }
            ?: error("missing input: $name -> options: ${job.workflowActivity.workflowActivity.inputs.map { it.workflowActivityParameter.name }}")
        return parameter.workflowActivityParameter.value
    }

    protected fun getOutputParameterValue(job: WorkflowJob, name: String): String? {
        val parameter = job.workflowActivity
            .workflowActivity
            .outputs
            .firstOrNull { it.workflowActivityParameter.name == name }
            ?: return null
        return parameter.workflowActivityParameter.value
    }

    protected suspend fun setContent(context: ActivityContext, job: WorkflowJob, file: File) {
        val upload =
            client.metadata.getMetadataContentUpload(job.metadata?.metadata?.id ?: error("missing metadata id"))
                ?: error("missing upload")
        client.files.upload(upload, file)
    }

    protected suspend fun getContentFile(context: ActivityContext, job: WorkflowJob): File =
        getContentFile(context, job.id, job.metadata?.metadata ?: error("missing metadata"))

    protected suspend fun getContentFile(context: ActivityContext, jobId: WorkflowJob.Id, metadata: Metadata): File {
        val content = client.metadata.getMetadataContent(metadata.id)
            ?: error("missing content")
        val file = withContext(Dispatchers.IO) {
            File.createTempFile(jobId.id, "." + content.type.split("/").last())
        }
        context.addFile(file)
        client.files.download(content.urls.download, file)
        return file
    }

    protected suspend fun getUrlFile(context: ActivityContext, job: WorkflowJob, url: String): File {
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
        val supplementaryKey = job.workflowActivity.workflowActivity.inputs
            .firstOrNull {
                it.workflowActivityParameter.name == key
            }?.workflowActivityParameter?.value ?: return false
        val supplementary = job.metadata?.metadata?.supplementary?.firstOrNull {
            it.metadataSupplementary.key == supplementaryKey
        }?.metadataSupplementary
        return supplementary != null
    }

    protected suspend fun getInputSupplementaryFile(context: ActivityContext, job: WorkflowJob, key: String): File {
        val supplementaryKey = job.workflowActivity.workflowActivity.inputs.firstOrNull {
            it.workflowActivityParameter.name == key
        }?.workflowActivityParameter?.value ?: key
        val supplementary = job.metadata?.metadata?.supplementary?.firstOrNull {
            it.metadataSupplementary.key == supplementaryKey
        }?.metadataSupplementary ?: error("missing supplementary: $key")
        val file = withContext(Dispatchers.IO) {
            File.createTempFile(job.id.id, supplementary.content.metadataSupplementaryContent.type.split("/").last())
        }
        context.addFile(file)
        client.files.download(supplementary.content.metadataSupplementaryContent.urls.download, file)
        return file
    }

    protected suspend fun getInputSupplementaryText(
        context: ActivityContext,
        job: WorkflowJob,
        key: String
    ): String {
        val file = getInputSupplementaryFile(context, job, key)
        return file.readText()
    }

    protected suspend inline fun <reified T> getInputSupplementary(
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

    protected suspend fun setSupplementaryContents(
        job: WorkflowJob,
        output: String,
        name: String,
        value: String,
        contentType: String,
        sourceId: String? = null,
        sourceIdentifier: String? = null
    ) {
        val metadataId = job.metadata?.metadata?.id ?: error("missing metadata id")
        val key = getOutputParameterValue(job, output) ?: name
        val supplementary =
            job.metadata.metadata.supplementary.firstOrNull { it.metadataSupplementary.key == key }?.metadataSupplementary
                ?: client.metadata.addSupplementary(
                    MetadataSupplementaryInput(
                        name = name,
                        contentType = contentType,
                        key = key,
                        metadataId = metadataId,
                        sourceId = Optional.presentIfNotNull(sourceId),
                        sourceIdentifier = Optional.presentIfNotNull(sourceIdentifier)
                    )
                ) ?: error("missing supplementary: $name")
        client.metadata.setSupplementaryTextContent(
            metadataId,
            supplementary.key,
            contentType,
            value
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
        val metadataId = job.metadata?.metadata?.id ?: error("missing metadata id")
        val key = getOutputParameterValue(job, output) ?: name
        val supplementary =
            job.metadata.metadata.supplementary.firstOrNull { it.metadataSupplementary.key == key }?.metadataSupplementary
                ?: client.metadata.addSupplementary(
                    MetadataSupplementaryInput(
                        name = name,
                        contentType = "application/json",
                        key = key,
                        metadataId = metadataId,
                        sourceId = Optional.presentIfNotNull(sourceId),
                        sourceIdentifier = Optional.presentIfNotNull(sourceIdentifier)
                    )
                ) ?: error("missing supplementary: $name")
        client.metadata.setSupplementaryTextContent(
            metadataId,
            supplementary.key,
            "application/json",
            if (value is String) value else Json.encodeToString(serializer, value)
        )
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

    protected inline fun <reified T> getConfiguration(job: WorkflowJob): T {
        return Json.decodeFromJsonElement<T>((job.workflowActivity.workflowActivity.configuration).toJsonElement())
    }

    protected suspend inline fun <reified T> setContext(job: WorkflowJob, value: T) {
        val data = Json.encodeToJsonElement(value).toAny()
        client.workflows.setWorkflowJobContext(job.id, data ?: emptyMap<String, Any>())
    }

    abstract suspend fun execute(context: ActivityContext, job: WorkflowJob)
}