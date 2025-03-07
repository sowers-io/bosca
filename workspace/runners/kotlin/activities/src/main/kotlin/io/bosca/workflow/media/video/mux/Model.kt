package io.bosca.workflow.media.video.mux

import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable

@Serializable
data class UploadResponse(val data: Upload)

@Serializable
data class Upload(
    val id: String,
    val url: String? = null,
    @SerialName("asset_id")
    val assetId: String? = null,
    val status: String
)


@Serializable
data class NewAssetSettings(
    @SerialName("playback_policy")
    val playbackPolicy: List<String>,
    @SerialName("encoding_tier")
    val encodingTier: String,
    val test: Boolean
)

@Serializable
data class UploadRequest(
    @SerialName("new_asset_settings")
    val newAssetSettings: NewAssetSettings,
    @SerialName("cors_origin")
    val corsOrigin: String
)

@Serializable
data class Asset(
    val tracks: List<AssetTrack>,
    val status: String,
    @SerialName("resolution_tier")
    val resolutionTier: String?,
    @SerialName("playback_ids")
    val playbackIds: List<PlaybackIds>?,
    val passthrough: String?,
    @SerialName("mp4_support")
    val mp4Support: String?,
    @SerialName("max_stored_resolution")
    val maxStoredResolution: String?,
    @SerialName("max_stored_frame_rate")
    val maxStoredFrameRate: Float?,
    @SerialName("master_access")
    val masterAccess: String?,
    val id: String,
    @SerialName("encoding_tier")
    val encodingTier: String?,
    @SerialName("video_quality")
    val videoQuality: String?,
    val duration: Float,
    @SerialName("aspect_ratio")
    val aspectRatio: String?,
)

@Serializable
data class AssetResponse(
    val data: Asset?
)

@Serializable
data class AssetTrack(
    val type: String,
    @SerialName("max_width")
    val maxWidth: Int?,
    @SerialName("max_height")
    val maxHeight: Int?,
    @SerialName("max_frame_rate")
    val maxFrameRate: Float?,
    val id: String,
    val duration: Float,
    @SerialName("max_channels")
    val maxChannels: Int?
)

@Serializable
data class PlaybackIds(
    val id: String,
    val policy: String
)

@Serializable
data class MuxRecord(
    val upload: Upload,
    val asset: Asset?,
    val uploaded: Boolean
)

@Serializable
data class MuxAttributes(
    @SerialName("playback_id")
    val playbackId: String,
    @SerialName("hls_url")
    val hlsUrl: String,
    @SerialName("aspect_ratio")
    val aspectRatio: String?,
    val duration: Float,
    @SerialName("video_quality")
    val videoQuality: String?,
)