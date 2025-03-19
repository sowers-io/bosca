package io.bosca.workflow.media.video.mux

import kotlinx.serialization.Serializable

@Serializable
data class MuxToken(
    val token: String,
    val id: String,
    val secret: String
)

@Serializable
data class MuxConfiguration(
    val test: Boolean = false,
    val token: MuxToken
) {

    companion object {
        const val KEY = "mux"
    }
}