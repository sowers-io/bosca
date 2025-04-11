package io.bosca.api

import com.apollographql.apollo.api.Optional
import io.bosca.graphql.*
import io.bosca.graphql.fragment.*
import io.bosca.graphql.fragment.Activity
import io.bosca.graphql.fragment.Model
import io.bosca.graphql.fragment.Prompt
import io.bosca.graphql.fragment.StorageSystem
import io.bosca.graphql.fragment.Trait
import io.bosca.graphql.fragment.Transition
import io.bosca.graphql.fragment.Workflow
import io.bosca.graphql.fragment.WorkflowActivity
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.fragment.WorkflowState
import io.bosca.graphql.type.*
import java.time.LocalDateTime
import java.time.ZonedDateTime
import java.util.Date

class Workflows(network: NetworkClient) : Api(network) {

    suspend fun getNextJob(queue: String): WorkflowJob? {
        val response = network.graphql.query(NextJobQuery(queue)).execute()
        response.validate()
        return response.data?.workflows?.nextJob?.workflowJob
    }

    suspend fun setMetadataWorkflowStateComplete(metadataId: String, status: String) {
        val response = network.graphql.mutation(
            SetMetadataWorkflowStateCompleteMutation(
                MetadataWorkflowCompleteState(
                    metadataId,
                    status
                )
            )
        ).execute()
        response.validate()
    }

    suspend fun setCollectionWorkflowStateComplete(metadataId: String, status: String) {
        val response = network.graphql.mutation(
            SetCollectionWorkflowStateCompleteMutation(
                CollectionWorkflowCompleteState(
                    metadataId,
                    status
                )
            )
        ).execute()
        response.validate()
    }

    suspend fun setMetadataWorkflowState(
        metadataId: String,
        stateId: String,
        status: String,
        immediate: Boolean = false
    ) {
        val response = network.graphql.mutation(
            SetMetadataWorkflowStateMutation(
                MetadataWorkflowState(
                    immediate,
                    metadataId,
                    stateId,
                    status
                )
            )
        ).execute()
        response.validate()
    }

    suspend fun setCollectionWorkflowState(
        collectionId: String,
        stateId: String,
        status: String,
        immediate: Boolean = false
    ) {
        val response = network.graphql.mutation(
            SetCollectionWorkflowStateMutation(
                CollectionWorkflowState(
                    collectionId,
                    immediate,
                    stateId,
                    status
                )
            )
        ).execute()
        response.validate()
    }

    suspend fun setWorkflowJobComplete(id: WorkflowJob.Id) {
        val response = network.graphql.mutation(
            SetWorkflowJobCompleteMutation(
                WorkflowJobIdInput(id.id, id.index, id.queue)
            )
        ).execute()
        response.validate()
    }

    suspend fun setWorkflowJobCheckin(id: WorkflowJob.Id) {
        val response = network.graphql.mutation(
            SetWorkflowJobCheckinMutation(
                WorkflowJobIdInput(id.id, id.index, id.queue)
            )
        ).execute()
        response.validate()
    }

    suspend fun setWorkflowJobDelayedUntil(id: WorkflowJob.Id, date: ZonedDateTime) {
        val response = network.graphql.mutation(
            SetWorkflowJobDelayedUntilMutation(
                WorkflowJobIdInput(id.id, id.index, id.queue),
                date
            )
        ).execute()
        response.validate()
    }

    suspend fun setWorkflowJobContext(id: WorkflowJob.Id, context: Any) {
        val response = network.graphql.mutation(
            SetWorkflowJobContextMutation(
                WorkflowJobIdInput(id.id, id.index, id.queue),
                context
            )
        ).execute()
        response.validate()
    }

    suspend fun setWorkflowJobFailed(id: WorkflowJob.Id, error: String) {
        val response = network.graphql.mutation(
            SetWorkflowJobFailedMutation(
                WorkflowJobIdInput(id.id, id.index, id.queue),
                error
            )
        ).execute()
        response.validate()
    }

    suspend fun getWorkflows(): List<Workflow> {
        val response = network.graphql.query(GetWorkflowsQuery()).execute()
        response.validate()
        return response.data?.workflows?.all?.map { it.workflow } ?: emptyList()
    }

    suspend fun add(workflow: WorkflowInput): Workflow? {
        val response = network.graphql.mutation(AddWorkflowMutation(workflow)).execute()
        response.validate()
        return response.data?.workflows?.add?.workflow
    }

    suspend fun get(id: String): Pair<Workflow, List<WorkflowActivity>>? {
        val response = network.graphql.query(GetWorkflowQuery(id)).execute()
        response.validate()
        val workflow = response.data?.workflows?.workflow ?: return null
        return Pair(workflow.workflow, workflow.activities.map { it.workflowActivity })
    }

    suspend fun edit(workflow: WorkflowInput) {
        val response = network.graphql.mutation(EditWorkflowMutation(workflow)).execute()
        response.validate()
    }

    suspend fun delete(id: String) {
        val response = network.graphql.mutation(DeleteWorkflowMutation(id)).execute()
        response.validate()
    }

    suspend fun getTraits(): List<Trait> {
        val response = network.graphql.query(GetTraitsQuery()).execute()
        response.validate()
        return response.data?.workflows?.traits?.all?.map { it.trait } ?: emptyList()
    }

    suspend fun getTrait(id: String): Trait? {
        val response = network.graphql.query(GetTraitQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.traits?.trait?.trait
    }

    suspend fun addTrait(input: TraitInput): Trait? {
        val response = network.graphql.mutation(AddTraitMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.traits?.add?.trait
    }

    suspend fun editTrait(input: TraitInput): Trait? {
        val response = network.graphql.mutation(EditTraitMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.traits?.edit?.trait
    }

    suspend fun deleteTrait(id: String) {
        val response = network.graphql.mutation(DeleteTraitMutation(id)).execute()
        response.validate()
    }

    suspend fun getWorkflowActivity(id: Int): WorkflowActivity? {
        val response = network.graphql.query(GetWorkflowActivityQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.workflowActivity?.workflowActivity
    }

    suspend fun getActivities(): List<Activity> {
        val response = network.graphql.query(GetActivitiesQuery()).execute()
        response.validate()
        return response.data?.workflows?.activities?.all?.map { it.activity } ?: emptyList()
    }

    suspend fun getActivity(id: String): Activity? {
        val response = network.graphql.query(GetActivityQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.activities?.activity?.activity
    }

    suspend fun addActivity(input: ActivityInput): Activity? {
        val response = network.graphql.mutation(AddActivityMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.activities?.add?.activity
    }

    suspend fun deleteActivity(id: String) {
        val response = network.graphql.mutation(DeleteActivityMutation(id)).execute()
        response.validate()
    }

    suspend fun editActivity(input: ActivityInput): Activity? {
        val response = network.graphql.mutation(EditActivityMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.activities?.edit?.activity
    }

    suspend fun getActivities(workflowId: String): List<WorkflowActivity> {
        val response = network.graphql.query(GetWorkflowActivitiesQuery(workflowId)).execute()
        response.validate()
        return response.data?.workflows?.workflow?.activities?.map { it.workflowActivity } ?: emptyList()
    }

    suspend fun getCollectionWorkflowPlans(id: String): List<WorkflowPlan> {
        val response = network.graphql.query(GetCollectionWorkflowPlansQuery(Optional.presentIfNotNull(id))).execute()
        response.validate()
        return response.data?.content?.collection?.workflow?.plans?.map { it.workflowPlan } ?: emptyList()
    }

    suspend fun getMetadataWorkflowPlans(id: String): List<WorkflowPlan> {
        val response = network.graphql.query(GetMetadataWorkflowPlansQuery(id)).execute()
        response.validate()
        return response.data?.content?.metadata?.workflow?.plans?.map { it.workflowPlan } ?: emptyList()
    }

    suspend fun getPrompts(): List<Prompt> {
        val response = network.graphql.query(GetPromptsQuery()).execute()
        response.validate()
        return response.data?.workflows?.prompts?.all?.map { it.prompt } ?: emptyList()
    }

    suspend fun getPrompt(id: String): Prompt? {
        val response = network.graphql.query(GetPromptQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.prompts?.prompt?.prompt
    }

    suspend fun addPrompt(input: PromptInput): Prompt? {
        val response = network.graphql.mutation(AddPromptMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.prompts?.add?.prompt
    }

    suspend fun editPrompt(id: String, input: PromptInput): Prompt? {
        val response = network.graphql.mutation(EditPromptMutation(id, input)).execute()
        response.validate()
        return response.data?.workflows?.prompts?.edit?.prompt
    }

    suspend fun deletePrompt(id: String) {
        val response = network.graphql.mutation(DeletePromptMutation(id)).execute()
        response.validate()
    }

    suspend fun getModels(): List<Model> {
        val response = network.graphql.query(GetModelsQuery()).execute()
        response.validate()
        return response.data?.workflows?.models?.all?.map { it.model } ?: emptyList()
    }

    suspend fun getModel(id: String): Model? {
        val response = network.graphql.query(GetModelQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.models?.model?.model
    }

    suspend fun addModel(model: ModelInput): Model? {
        val response = network.graphql.mutation(AddModelMutation(model)).execute()
        response.validate()
        return response.data?.workflows?.models?.add?.model
    }

    suspend fun editModel(id: String, model: ModelInput): Model? {
        val response = network.graphql.mutation(EditModelMutation(id, model)).execute()
        response.validate()
        return response.data?.workflows?.models?.edit?.model
    }

    suspend fun deleteModel(id: String) {
        val response = network.graphql.mutation(DeleteModelMutation(id)).execute()
        response.validate()
    }

    suspend fun getTransitions(): List<Transition> {
        val response = network.graphql.query(GetTransitionsQuery()).execute()
        response.validate()
        return response.data?.workflows?.transitions?.all?.map { it.transition } ?: emptyList()
    }

    suspend fun getTransition(fromStateId: String, toStateId: String): Transition? {
        val response = network.graphql.query(GetTransitionQuery(fromStateId, toStateId)).execute()
        response.validate()
        return response.data?.workflows?.transitions?.transition?.transition
    }

    suspend fun addTransition(transition: TransitionInput): Transition? {
        val response = network.graphql.mutation(AddTransitionMutation(transition)).execute()
        response.validate()
        return response.data?.workflows?.transitions?.add?.transition
    }

    suspend fun editTransition(transition: TransitionInput): Transition? {
        val response = network.graphql.mutation(EditTransitionMutation(transition)).execute()
        response.validate()
        return response.data?.workflows?.transitions?.edit?.transition
    }

    suspend fun deleteTransition(fromStateId: String, toStateId: String) {
        val response = network.graphql.mutation(DeleteTransitionMutation(fromStateId, toStateId)).execute()
        response.validate()
    }

    suspend fun beginMetadataTransition(id: String, version: Int, state: String, status: String, restart: Boolean = false, waitForCompletion: Boolean = false) {
        val response = network.graphql.mutation(BeginMetadataTransitionMutation(id, version, state, status, restart, waitForCompletion)).execute()
        response.validate()
    }

    suspend fun beginCollectionTransition(id: String, state: String, status: String, restart: Boolean = false, waitForCompletion: Boolean = false) {
        val response = network.graphql.mutation(BeginCollectionTransitionMutation(id, state, status, restart, waitForCompletion)).execute()
        response.validate()
    }

    suspend fun enqueueChildWorkflows(workflowIds: List<String>, id: WorkflowJob.Id) {
        val response = network.graphql.mutation(
            EnqueueChildWorkflowsMutation(
                workflowIds,
                WorkflowJobIdInput(id.id, id.index, id.queue),
            )
        ).execute()
        response.validate()
    }

    suspend fun enqueueChildWorkflow(
        workflowId: String,
        id: WorkflowJob.Id,
        configuration: List<WorkflowConfigurationInput>? = null
    ) {
        val response = network.graphql.mutation(
            EnqueueChildWorkflowMutation(
                workflowId = workflowId,
                jobId = WorkflowJobIdInput(id.id, id.index, id.queue),
                configurations = configuration ?: emptyList(),
            )
        ).execute()
        response.validate()
    }

    suspend fun enqueueMetadataWorkflow(
        workflowId: String,
        metadataId: String,
        version: Int,
        configuration: List<WorkflowConfigurationInput>? = null
    ) {
        val response = network.graphql.mutation(
            EnqueueWorkflowMutation(
                workflowId = workflowId,
                collectionId = Optional.absent(),
                profileId = Optional.absent(),
                metadataId = Optional.presentIfNotNull(metadataId),
                version = Optional.presentIfNotNull(version),
                configurations = Optional.presentIfNotNull(configuration),
            )
        ).execute()
        response.validate()
    }

    suspend fun enqueueCollectionWorkflow(
        workflowId: String,
        collectionId: String,
        configuration: List<WorkflowConfigurationInput>? = null
    ) {
        val response = network.graphql.mutation(
            EnqueueWorkflowMutation(
                workflowId = workflowId,
                collectionId = Optional.presentIfNotNull(collectionId),
                profileId = Optional.absent(),
                metadataId = Optional.absent(),
                version = Optional.absent(),
                configurations = Optional.presentIfNotNull(configuration),
            )
        ).execute()
        response.validate()
    }

    suspend fun enqueueProfileWorkflow(
        workflowId: String,
        profileId: String,
        configuration: List<WorkflowConfigurationInput>? = null
    ) {
        val response = network.graphql.mutation(
            EnqueueWorkflowMutation(
                workflowId = workflowId,
                collectionId = Optional.absent(),
                profileId = Optional.presentIfNotNull(profileId),
                metadataId = Optional.absent(),
                version = Optional.absent(),
                configurations = Optional.presentIfNotNull(configuration),
            )
        ).execute()
        response.validate()
    }

    suspend fun getStates(): List<WorkflowState> {
        val response = network.graphql.query(GetStatesQuery()).execute()
        response.validate()
        return response.data?.workflows?.states?.all?.map { it.workflowState } ?: emptyList()
    }

    suspend fun getState(id: String): WorkflowState? {
        val response = network.graphql.query(GetStateQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.states?.state?.workflowState
    }

    suspend fun addState(state: WorkflowStateInput): WorkflowState? {
        val response = network.graphql.mutation(AddStateMutation(state)).execute()
        response.validate()
        return response.data?.workflows?.states?.add?.workflowState
    }

    suspend fun editState(state: WorkflowStateInput): WorkflowState? {
        val response = network.graphql.mutation(EditStateMutation(state)).execute()
        response.validate()
        return response.data?.workflows?.states?.edit?.workflowState
    }

    suspend fun deleteState(id: String) {
        val response = network.graphql.mutation(DeleteStateMutation(id)).execute()
        response.validate()
    }

    suspend fun getStorageSystems(): List<GetStorageSystemsQuery.All> {
        val response = network.graphql.query(GetStorageSystemsQuery()).execute()
        response.validate()
        return response.data?.workflows?.storageSystems?.all ?: emptyList()
    }

    suspend fun getStorageSystem(id: String): StorageSystem? {
        val response = network.graphql.query(GetStorageSystemQuery(id)).execute()
        response.validate()
        return response.data?.workflows?.storageSystems?.storageSystem?.storageSystem
    }

    suspend fun addStorageSystem(input: StorageSystemInput): StorageSystem? {
        val response = network.graphql.mutation(AddStorageSystemMutation(input)).execute()
        response.validate()
        return response.data?.workflows?.storageSystems?.add?.storageSystem
    }

    suspend fun editStorageSystem(id: String, input: StorageSystemInput): StorageSystem? {
        val response = network.graphql.mutation(EditStorageSystemMutation(id, input)).execute()
        response.validate()
        return response.data?.workflows?.storageSystems?.edit?.storageSystem
    }

    suspend fun deleteStorageSystem(id: String) {
        val response = network.graphql.mutation(DeleteStorageSystemMutation(id)).execute()
        response.validate()
    }
}