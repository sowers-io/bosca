package io.bosca.workflow.media.video.mux

import io.bosca.api.Client
import io.bosca.api.KeyValue
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.api.executeAsync
import io.bosca.util.DefaultKeys
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import java.util.*

class Uploader(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Mux Uploader",
            description = "Upload a video to Mux",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    private suspend fun newRequestBuilder(): Request.Builder {
        val tokenId = client.configurations.get<KeyValue>(DefaultKeys.MUX_TOKEN_ID) ?: error("MUX_TOKEN_ID environment variable is missing")
        val tokenSecret = client.configurations.get<KeyValue>(DefaultKeys.MUX_TOKEN_SECRET) ?: error("MUX_TOKEN_SECRET environment variable is missing")
        return Request.Builder()
            .addHeader(
                "Authorization",
                "Basic ${Base64.getEncoder().encodeToString("$tokenId:$tokenSecret".toByteArray())}"
            )
    }

    private suspend fun uploadFile(context: ActivityContext, job: WorkflowJob, record: MuxRecord): MuxRecord {
        val content = getContentFile(context, job)
        content.inputStream().use { input ->
            val size = content.length()
            val bufSize = 1024 * 1024 * 30
            val buffer = ByteArray(bufSize)
            var offset = 0
            while (offset < size) {
                val read = input.read(buffer, offset, bufSize)
                val request = newRequestBuilder()
                    .url("https://api.mux.com/video/v1/uploads/${record.upload.id}")
                    .put(buffer.toRequestBody("application/octet-stream".toMediaType()))
                    .addHeader("Content-Length", "$read")
                    .addHeader("Content-Range", "bytes $offset-${offset + read - 1}/$size")
                    .build()
                val response = client.network.http.newCall(request).executeAsync()
                offset += read
                if (response.code != 308 && !response.isSuccessful) {
                    val text = response.body!!.string()
                    throw Exception("failed to upload: ${response.code} $text")
                }
            }
        }
        @Suppress("NAME_SHADOWING")
        val record = record.copy(uploaded = true)
        setContext(job, record)
        return record
    }

    private suspend fun initContext(job: WorkflowJob): MuxRecord {
        val upload = UploadRequest(
            newAssetSettings = NewAssetSettings(
                playbackPolicy = listOf("public"),
                encodingTier = "smart",
                test = client.configurations.get<KeyValue>(DefaultKeys.MUX_TEST)?.value == "true",
            ),
            corsOrigin = "*",
        )
        val request = newRequestBuilder()
            .url("https://api.mux.com/video/v1/uploads")
            .post(Json.encodeToString(upload).toRequestBody("application/json".toMediaType()))
            .build()
        val response = client.network.http.newCall(request).executeAsync().let {
            Json.decodeFromString<UploadResponse>(it.body!!.string())
        }
        val record = MuxRecord(
            upload = response.data,
            asset = null,
            uploaded = false
        )
        setContext(job, record)
        return record
    }

    private suspend fun getUpload(record: MuxRecord): Upload {
        val response = client.network.http.newCall(
            newRequestBuilder()
                .url("https://api.mux.com/video/v1/uploads/${record.upload.id}")
                .get()
                .build()
        ).executeAsync()
        val uploadResponse = Json.decodeFromString<UploadResponse>(response.body!!.string())
        if (uploadResponse.data.assetId == null) {
            error("asset id is missing")
        }
        return uploadResponse.data
    }

    private suspend fun getAsset(upload: Upload): Asset {
        val response = client.network.http.newCall(
            newRequestBuilder()
                .url("https://api.mux.com/video/v1/assets/${upload.assetId}")
                .get()
                .build()
        ).executeAsync()
        val assetResponse = Json.decodeFromString<AssetResponse>(response.body!!.string())
        return assetResponse.data ?: error("data is missing")
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        var record = if (job.context == null) {
            initContext(job)
        } else {
            getContext(job)
        }

        record.asset?.let {
            it.playbackIds?.let { playbackIds ->
                if (playbackIds.isNotEmpty()) {
                    return
                }
            }
        }

        if (!record.uploaded) {
            record = uploadFile(context, job, record)
        }

        val upload = getUpload(record)
        record = record.copy(asset = getAsset(upload))
        setContext(job, record)

        val asset = record.asset ?: error("asset missing")
        val playbackId = asset.playbackIds?.firstOrNull()?.id ?: error("playback id missing")
        val attributes = MuxAttributes(
            playbackId = playbackId,
            hlsUrl = "https://stream.mux.com/$playbackId.m3u8",
            aspectRatio = asset.videoQuality,
            duration = asset.duration,
            videoQuality = asset.videoQuality,
        )

        setAttribute(job, "mux", attributes)
    }

    companion object {

        const val ID = "video.mux.uploader"
    }
}