package io.bosca.api

import io.bosca.graphql.fragment.CollectionSupplementaryContentDownload
import io.bosca.graphql.fragment.MetadataContentDownload
import io.bosca.graphql.fragment.MetadataContentUpload
import io.bosca.graphql.fragment.MetadataSupplementaryContentDownload
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.MultipartBody
import okhttp3.MultipartBody.Companion.FORM
import okhttp3.Request
import okhttp3.RequestBody.Companion.asRequestBody
import java.io.File
import java.nio.file.Files


class Files(network: NetworkClient) : Api(network) {

    suspend fun download(download: MetadataSupplementaryContentDownload.Download, file: File) {
        val request = Request.Builder()
            .url(download.url)
            .apply {
                download.headers.forEach { header ->
                    addHeader(header.name, header.value)
                }
            }
            .build()
        execute(request, file)
    }

    suspend fun download(download: CollectionSupplementaryContentDownload.Download, file: File) {
        val request = Request.Builder()
            .url(download.url)
            .apply {
                download.headers.forEach { header ->
                    addHeader(header.name, header.value)
                }
            }
            .build()
        execute(request, file)
    }

    suspend fun download(download: MetadataContentDownload.Download, file: File) {
        val request = Request.Builder()
            .url(download.url)
            .apply {
                download.headers.forEach { header ->
                    addHeader(header.name, header.value)
                }
            }
            .build()
        execute(request, file)
    }

    suspend fun upload(upload: MetadataContentUpload.Upload, file: File) {
        val mimeType = withContext(Dispatchers.IO) {
            Files.probeContentType(file.toPath())
        } ?: "application/octet-stream"
        val request = Request.Builder()
            .post(
                MultipartBody.Builder()
                    .setType(FORM)
                    .addFormDataPart("file", file.name, file.asRequestBody(mimeType.toMediaType()))
                    .build()
            )
            .url(upload.url)
            .apply {
                upload.headers.forEach { header ->
                    addHeader(header.name, header.value)
                }
            }
            .build()
        execute(request, file)
    }

    suspend fun download(url: String, file: File, authorization: String? = null) {
        val request = Request.Builder().url(url)
        authorization?.let {
            request.header("Authorization", it)
        }
        execute(request.build(), file)
    }

    private suspend fun execute(request: Request, file: File) {
        network.http.newCall(request).executeAsync().use { response ->
            if (response.code == 404) {
                return
            }
            if (!response.isSuccessful) throw Exception("Unexpected code $response")
            if (response.body == null) throw Exception("Missing body")
            withContext(Dispatchers.IO) {
                file.writeBytes(response.body!!.bytes())
            }
        }
    }
}