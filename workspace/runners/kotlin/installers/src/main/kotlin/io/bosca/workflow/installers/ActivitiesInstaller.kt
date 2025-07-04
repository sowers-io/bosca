package io.bosca.workflow.installers

import ThumbnailExtractor
import io.bosca.api.Client
import io.bosca.installer.Installer
import io.bosca.workflow.ActivityRegistry
import io.bosca.workflow.ai.ExecutePrompt
import io.bosca.workflow.json.Jq

import io.bosca.workflow.metadata.Traits as MetadataTraits
import io.bosca.workflow.metadata.BeginTransitionTo as MetadataBeginTransitionTo
import io.bosca.workflow.metadata.TransitionTo as MetadataTransitionTo
import io.bosca.workflow.metadata.RestartTransitionTo as MetadataRestartTransitionTo
import io.bosca.workflow.metadata.SetPublic as MetadataSetPublic
import io.bosca.workflow.metadata.SetReady as MetadataSetReady
import io.bosca.workflow.metadata.DownloadSourceUrl as MetadataDownloadSourceUrl
import io.bosca.workflow.metadata.PermanentlyDelete as MetadataPermanentlyDelete
import io.bosca.workflow.metadata.PublishGuide as MetadataPublishGuide
import io.bosca.workflow.metadata.DeleteSupplementary as MetadataDeleteSupplementary
import io.bosca.workflow.metadata.DeleteAllSupplementary as MetadataDeleteAllSupplementary
import io.bosca.workflow.metadata.DeleteAllPlanSupplementary as MetadataDeleteAllPlanSupplementary

import io.bosca.workflow.collection.Traits as CollectionTraits
import io.bosca.workflow.collection.BeginTransitionTo as CollectionBeginTransitionTo
import io.bosca.workflow.collection.TransitionTo as CollectionTransitionTo
import io.bosca.workflow.collection.RestartTransitionTo as CollectionRestartTransitionTo
import io.bosca.workflow.collection.SetPublic as CollectionSetPublic
import io.bosca.workflow.collection.SetReady as CollectionSetReady
import io.bosca.workflow.collection.GenerateList as CollectionGenerateList
import io.bosca.workflow.collection.PermanentlyDelete as CollectionPermanentlyDelete
import io.bosca.workflow.collection.DeleteSupplementary as CollectionDeleteSupplementary
import io.bosca.workflow.collection.DeleteAllSupplementary as CollectionDeleteAllSupplementary
import io.bosca.workflow.collection.DeleteAllPlanSupplementary as CollectionDeleteAllPlanSupplementary

import io.bosca.workflow.ai.embeddings.GenerateEmbeddings
import io.bosca.workflow.email.EmailActivity
import io.bosca.workflow.general.ActivityInstaller
import io.bosca.workflow.general.Delay
import io.bosca.workflow.general.If
import io.bosca.workflow.json.JSONata
import io.bosca.workflow.media.image.IfSquare
import io.bosca.workflow.media.image.ImageRelationshipResizer
import io.bosca.workflow.media.image.ImageResizer
import io.bosca.workflow.metadata.*
import io.bosca.workflow.search.AddToIndex
import io.bosca.workflow.search.DeleteFromIndex
import io.bosca.workflow.search.DeleteFromIndexes
import io.bosca.workflow.storage.InitializeIndex
import io.bosca.workflow.storage.RebuildData
import java.io.File
import io.bosca.workflow.media.video.mux.Uploader as MuxUploader

class ActivitiesInstaller(client: Client) : Installer, ActivityRegistry {

    private val activities = listOf(
        Jq(client),

        Delay(client),

        EmailActivity(client),

        CollectionToJson(client),
        MetadataToJson(client),
        ProfileToJson(client),

        InitializeIndex(client),
        AddToIndex(client),
        DeleteFromIndex(client),
        DeleteFromIndexes(client),
        RebuildData(client),

        ActivityInstaller(client),

        MetadataTraits(client),
        MetadataTransitionTo(client),
        MetadataBeginTransitionTo(client),
        MetadataRestartTransitionTo(client),
        MetadataSetPublic(client),
        MetadataSetReady(client),
        MetadataDownloadSourceUrl(client),
        MetadataPermanentlyDelete(client),
        MetadataPublishGuide(client),
        MetadataDeleteSupplementary(client),
        MetadataDeleteAllSupplementary(client),
        MetadataDeleteAllPlanSupplementary(client),
        PublishMetadata(client),

        PublishRelationships(client),

        ExecuteChildWorkflow(client),

        CollectionTraits(client),
        CollectionTransitionTo(client),
        CollectionBeginTransitionTo(client),
        CollectionRestartTransitionTo(client),
        CollectionSetPublic(client),
        CollectionSetReady(client),
        CollectionGenerateList(client),
        CollectionPermanentlyDelete(client),
        CollectionDeleteSupplementary(client),
        CollectionDeleteAllSupplementary(client),
        CollectionDeleteAllPlanSupplementary(client),
        PublishCollection(client),

        IfSquare(client),
        ImageResizer(client),
        ImageRelationshipResizer(client),

        MuxUploader(client),
        
        DocumentToText(client),

        ThumbnailExtractor(client),

        SetAttribute(client),

        ExecutePrompt(client),

        GenerateEmbeddings(client),

        JSONata(client),

        If(client)
    ).associateBy { it.id }

    override suspend fun install(client: Client, directory: File) {
        install(client)
    }

    override suspend fun install(client: Client) {
        val currentActivities = client.workflows.getActivities().associateBy { it.id }
        for (activity in activities.values) {
            if (currentActivities.containsKey(activity.id)) {
                client.workflows.editActivity(activity.toActivityDefinition())
            } else {
                client.workflows.addActivity(activity.toActivityDefinition())
            }
        }
    }

    override fun getActivity(id: String) = activities[id]
}