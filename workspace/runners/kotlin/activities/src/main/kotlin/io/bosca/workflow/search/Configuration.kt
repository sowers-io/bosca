package io.bosca.workflow.search

import com.meilisearch.sdk.Config
import com.meilisearch.sdk.json.JacksonJsonHandler
import kotlinx.serialization.Serializable

@Serializable
data class MeilisearchConfiguration(
    val hostUrl: String? = null,
    val apiKey: String? = null,
    val experimental: ExperimentalConfiguration? = null
)

@Serializable
data class ExperimentalConfiguration(
    val chatCompletions: Boolean = false,
)

fun newMeilisearchConfig(meilisearchConfiguration: MeilisearchConfiguration? = null): Config = Config(
    meilisearchConfiguration?.hostUrl ?: System.getenv("SEARCH_URL"),
    meilisearchConfiguration?.apiKey ?: System.getenv("SEARCH_KEY"),
    JacksonJsonHandler()
)