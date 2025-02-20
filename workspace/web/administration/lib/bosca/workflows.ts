import {
  AddPromptDocument,
  AddTraitDocument,
  DeletePromptDocument,
  DeleteTraitDocument,
  EditPromptDocument,
  EditTraitDocument,
  EnqueueWorkflowDocument,
  GetPromptDocument,
  GetTraitDocument,
  type ModelFragment,
  type PromptFragment,
  type PromptInput,
  type StorageSystemFragment,
  type TraitFragment,
  type TraitInput,
  type TransitionFragment,
  type WorkflowConfigurationInput, type WorkflowExecutionId,
  type WorkflowFragment,
  type WorkflowPlanFragment,
  type WorkflowStateFragment,
} from '~/lib/graphql/graphql'
import { Api } from '~/lib/bosca/api'
import {
  GetCollectionWorkflowPlansDocument,
  GetMetadataWorkflowPlansDocument,
  GetModelsDocument,
  GetPromptsDocument,
  GetStatesDocument,
  GetStorageSystemsDocument,
  GetTraitsDocument,
  GetTransitionsDocument,
  GetWorkflowsDocument,
} from '~/lib/graphql/graphql'
import type { AsyncData } from '#app/composables/asyncData'
import type { NetworkClient } from '~/lib/bosca/networkclient'

export class Workflows<T extends NetworkClient> extends Api<T> {
  constructor(network: T) {
    super(network)
  }

  getAllAsyncData(): AsyncData<Array<WorkflowFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetWorkflowsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.all as Array<WorkflowFragment>
      },
    )
  }

  //
  // suspend fun getWorkflows(): List<Workflow> {
  //     val response = network.client.query(GetWorkflowsQuery()).execute()
  //     response.validate()
  //     return response.data?.workflows?.all?.map { it.workflow } ?: emptyList()
  // }
  //
  // suspend fun add(workflow: WorkflowInput): Workflow? {
  //     val response = network.client.mutation(AddWorkflowMutation(workflow)).execute()
  //     response.validate()
  //     return response.data?.workflows?.add?.workflow
  // }
  //
  // suspend fun get(id: String): Pair<Workflow, List<WorkflowActivity>>? {
  //     val response = network.client.query(GetWorkflowQuery(id)).execute()
  //     response.validate()
  //     val workflow = response.data?.workflows?.workflow ?: return null
  //     return Pair(workflow.workflow, workflow.activities.map { it.workflowActivity })
  // }
  //
  // suspend fun edit(workflow: WorkflowInput) {
  //     val response = network.client.mutation(EditWorkflowMutation(workflow)).execute()
  //     response.validate()
  // }
  //
  // suspend fun delete(id: String) {
  //     val response = network.client.mutation(DeleteWorkflowMutation(id)).execute()
  //     response.validate()
  // }
  //

  getTraitsAsyncData(): AsyncData<Array<TraitFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetTraitsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.traits.all as Array<TraitFragment>
      },
    )
  }

  getTraitAsyncData(id: string): AsyncData<TraitFragment | null, any> {
    return this.executeAndTransformAsyncData(
      GetTraitDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.workflows.traits.trait as TraitFragment
      },
    )
  }

  async addTrait(input: TraitInput) {
    await this.network.execute(AddTraitDocument, { input })
  }

  async editTrait(input: TraitInput) {
    await this.network.execute(EditTraitDocument, { input })
  }

  async deleteTrait(id: string) {
    await this.network.execute(DeleteTraitDocument, { id })
  }

  // suspend fun getWorkflowActivity(id: Int): WorkflowActivity? {
  //     val response = network.client.query(GetWorkflowActivityQuery(id)).execute()
  //     response.validate()
  //     return response.data?.workflows?.workflowActivity?.workflowActivity
  // }
  //
  // suspend fun getActivities(): List<Activity> {
  //     val response = network.client.query(GetActivitiesQuery()).execute()
  //     response.validate()
  //     return response.data?.workflows?.activities?.all?.map { it.activity } ?: emptyList()
  // }
  //
  // suspend fun getActivity(id: String): Activity? {
  //     val response = network.client.query(GetActivityQuery(id)).execute()
  //     response.validate()
  //     return response.data?.workflows?.activities?.activity?.activity
  // }
  //
  // suspend fun addActivity(input: ActivityInput): Activity? {
  //     val response = network.client.mutation(AddActivityMutation(input)).execute()
  //     response.validate()
  //     return response.data?.workflows?.activities?.add?.activity
  // }
  //
  // suspend fun deleteActivity(id: String) {
  //     val response = network.client.mutation(DeleteActivityMutation(id)).execute()
  //     response.validate()
  // }
  //
  // suspend fun editActivity(input: ActivityInput): Activity? {
  //     val response = network.client.mutation(EditActivityMutation(input)).execute()
  //     response.validate()
  //     return response.data?.workflows?.activities?.edit?.activity
  // }
  //
  // suspend fun getActivities(workflowId: String): List<WorkflowActivity> {
  //     val response = network.client.query(GetWorkflowActivitiesQuery(workflowId)).execute()
  //     response.validate()
  //     return response.data?.workflows?.workflow?.activities?.map { it.workflowActivity } ?: emptyList()
  // }

  getMetadataWorkflowPlansAsyncData(
    id: string,
  ): AsyncData<Array<WorkflowPlanFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetMetadataWorkflowPlansDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.metadata?.workflow.plans as Array<
          WorkflowPlanFragment
        >
      },
    )
  }

  getCollectionWorkflowPlansAsyncData(
    id: string,
  ): AsyncData<Array<WorkflowPlanFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetCollectionWorkflowPlansDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.content.collection?.workflow.plans as Array<
          WorkflowPlanFragment
        >
      },
    )
  }

  getPromptsAsyncData(): AsyncData<Array<PromptFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetPromptsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.prompts.all as Array<PromptFragment>
      },
    )
  }

  getPromptAsyncData(id: string): AsyncData<PromptFragment | null, any> {
    return this.executeAndTransformAsyncData(
      GetPromptDocument,
      { id },
      (data) => {
        if (!data) return null
        return data.workflows.prompts.prompt as PromptFragment
      },
    )
  }

  async addPrompt(
    prompt: PromptInput,
  ): Promise<string> {
    const response = await this.network.execute(AddPromptDocument, { prompt })
    return response!.workflows!.prompts!.add!.id
  }

  async editPrompt(
    id: string,
    input: PromptInput,
  ): Promise<void> {
    await this.network.execute(EditPromptDocument, { id, input })
  }

  async deletePrompt(id: string) {
    await this.network.execute(DeletePromptDocument, { id })
  }

  getModelsAsyncData(): AsyncData<Array<ModelFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetModelsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.models.all as Array<ModelFragment>
      },
    )
  }
  //
  // suspend fun getModel(id: String): Model? {
  //     val response = network.client.query(GetModelQuery(id)).execute()
  //     response.validate()
  //     return response.data?.workflows?.models?.model?.model
  // }
  //
  // suspend fun addModel(model: ModelInput): Model? {
  //     val response = network.client.mutation(AddModelMutation(model)).execute()
  //     response.validate()
  //     return response.data?.workflows?.models?.add?.model
  // }
  //
  // suspend fun editModel(id: String, model: ModelInput): Model? {
  //     val response = network.client.mutation(EditModelMutation(id, model)).execute()
  //     response.validate()
  //     return response.data?.workflows?.models?.edit?.model
  // }
  //
  // suspend fun deleteModel(id: String) {
  //     val response = network.client.mutation(DeleteModelMutation(id)).execute()
  //     response.validate()
  // }
  //
  getTransitionsAsyncData(): AsyncData<Array<TransitionFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetTransitionsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.transitions.all as Array<TransitionFragment>
      },
    )
  }
  //
  // suspend fun getTransition(fromStateId: String, toStateId: String): Transition? {
  //     val response = network.client.query(GetTransitionQuery(fromStateId, toStateId)).execute()
  //     response.validate()
  //     return response.data?.workflows?.transitions?.transition?.transition
  // }
  //
  // suspend fun addTransition(transition: TransitionInput): Transition? {
  //     val response = network.client.mutation(AddTransitionMutation(transition)).execute()
  //     response.validate()
  //     return response.data?.workflows?.transitions?.add?.transition
  // }
  //
  // suspend fun editTransition(transition: TransitionInput): Transition? {
  //     val response = network.client.mutation(EditTransitionMutation(transition)).execute()
  //     response.validate()
  //     return response.data?.workflows?.transitions?.edit?.transition
  // }
  //
  // suspend fun deleteTransition(fromStateId: String, toStateId: String) {
  //     val response = network.client.mutation(DeleteTransitionMutation(fromStateId, toStateId)).execute()
  //     response.validate()
  // }
  //

  async enqueueMetadataWorkflow(
    workflowId: string,
    metadataId: string,
    version: number,
    configuration: Array<WorkflowConfigurationInput> = [],
  ): Promise<WorkflowExecutionId> {
    const response = await this.network.execute(EnqueueWorkflowDocument, {
      workflowId,
      metadataId,
      version,
      configuration,
    })
    return response!.workflows!.enqueueWorkflow!
  }

  // suspend fun enqueueMetadataWorkflow(workflowId: String, metadataId: String, version: Int, configuration: List<WorkflowConfigurationInput>? = null) {
  //     val response = network.client.mutation(EnqueueWorkflowMutation(
  //         workflowId,
  //         Optional.absent(),
  //         Optional.presentIfNotNull(metadataId),
  //         Optional.presentIfNotNull(version),
  //         Optional.presentIfNotNull(configuration),
  //     )).execute()
  //     response.validate()
  // }
  //
  // suspend fun enqueueCollectionWorkflow(workflowId: String, collectionId: String, configuration: List<WorkflowConfigurationInput>? = null) {
  //     val response = network.client.mutation(EnqueueWorkflowMutation(
  //         workflowId,
  //         Optional.presentIfNotNull(collectionId),
  //         Optional.absent(),
  //         Optional.absent(),
  //         Optional.presentIfNotNull(configuration),
  //     )).execute()
  //     response.validate()
  // }
  //
  getStatesAsyncData(): AsyncData<Array<WorkflowStateFragment> | null, any> {
    return this.executeAndTransformAsyncData(
      GetStatesDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.states.all as Array<WorkflowStateFragment>
      },
    )
  }

  // suspend fun getState(id: String): WorkflowState? {
  //     val response = network.client.query(GetStateQuery(id)).execute()
  //     response.validate()
  //     return response.data?.workflows?.states?.state?.workflowState
  // }
  //
  // suspend fun addState(state: WorkflowStateInput): WorkflowState? {
  //     val response = network.client.mutation(AddStateMutation(state)).execute()
  //     response.validate()
  //     return response.data?.workflows?.states?.add?.workflowState
  // }
  //
  // suspend fun editState(state: WorkflowStateInput): WorkflowState? {
  //     val response = network.client.mutation(EditStateMutation(state)).execute()
  //     response.validate()
  //     return response.data?.workflows?.states?.edit?.workflowState
  // }
  //
  // suspend fun deleteState(id: String) {
  //     val response = network.client.mutation(DeleteStateMutation(id)).execute()
  //     response.validate()
  // }
  //
  async getStorageSystems(): Promise<StorageSystemFragment[]> {
    const response = await this.network.execute(GetStorageSystemsDocument)
    return response!.workflows!.storageSystems!.all as StorageSystemFragment[]
  }

  getStorageSystemsAsyncData(): AsyncData<
    Array<StorageSystemFragment> | null,
    any
  > {
    return this.executeAndTransformAsyncData(
      GetStorageSystemsDocument,
      null,
      (data) => {
        if (!data) return null
        return data.workflows.storageSystems.all as StorageSystemFragment[]
      },
    )
  }
  //
  // suspend fun getStorageSystem(id: String): StorageSystem? {
  //     val response = network.client.query(GetStorageSystemQuery(id)).execute()
  //     response.validate()
  //     return response.data?.workflows?.storageSystems?.storageSystem?.storageSystem
  // }
  //
  // suspend fun addStorageSystem(input: StorageSystemInput): StorageSystem? {
  //     val response = network.client.mutation(AddStorageSystemMutation(input)).execute()
  //     response.validate()
  //     return response.data?.workflows?.storageSystems?.add?.storageSystem
  // }
  //
  // suspend fun editStorageSystem(id: String, input: StorageSystemInput): StorageSystem? {
  //     val response = network.client.mutation(EditStorageSystemMutation(id, input)).execute()
  //     response.validate()
  //     return response.data?.workflows?.storageSystems?.edit?.storageSystem
  // }
  //
  // suspend fun deleteStorageSystem(id: String) {
  //     val response = network.client.mutation(DeleteStorageSystemMutation(id)).execute()
  //     response.validate()
  // }
}
