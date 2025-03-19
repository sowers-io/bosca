package io.bosca.workflow.search

import com.meilisearch.sdk.Config
import com.meilisearch.sdk.json.JacksonJsonHandler
import kotlinx.serialization.Serializable

@Serializable
data class Configuration(val host: String, val apiKey: String) {

    fun toConfig(): Config = Config(
        host,
        apiKey,
        JacksonJsonHandler()
    )
}