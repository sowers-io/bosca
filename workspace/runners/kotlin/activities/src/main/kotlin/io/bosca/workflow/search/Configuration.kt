package io.bosca.workflow.search

import com.meilisearch.sdk.Config
import com.meilisearch.sdk.json.JacksonJsonHandler

fun newMeilisearchConfig(): Config = Config(
    System.getenv("SEARCH_URL"),
    System.getenv("SEARCH_KEY"),
    JacksonJsonHandler()
)