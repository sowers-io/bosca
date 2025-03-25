use crate::datastores::cache::cache::{BoscaCache, BoscaCacheInterface};
use crate::datastores::cache::manager::BoscaCacheManager;
use crate::datastores::cache::tiered_cache::TieredCacheType;
use crate::models::workflow::activities::{
    Activity, ActivityParameter, WorkflowActivity, WorkflowActivityModel,
    WorkflowActivityParameter, WorkflowActivityPrompt, WorkflowActivityStorageSystem,
};
use crate::models::workflow::models::Model;
use crate::models::workflow::prompts::Prompt;
use crate::models::workflow::states::WorkflowState;
use crate::models::workflow::storage_system_models::StorageSystemModel;
use crate::models::workflow::storage_systems::StorageSystem;
use crate::models::workflow::traits::Trait;
use crate::models::workflow::transitions::Transition;
use crate::models::workflow::workflows::Workflow;
use uuid::Uuid;

#[derive(Clone)]
pub struct WorkflowCache {
    trait_cache: BoscaCache<String, Trait>,

    trait_workflow_ids_cache: BoscaCache<String, Vec<String>>,
    prompt_cache: BoscaCache<Uuid, Prompt>,
    model_cache: BoscaCache<Uuid, Model>,
    storage_system_cache: BoscaCache<Uuid, StorageSystem>,
    storage_system_models_cache: BoscaCache<Uuid, Vec<StorageSystemModel>>,
    state_cache: BoscaCache<String, WorkflowState>,
    transition_cache: BoscaCache<String, Transition>,

    activity_cache: BoscaCache<String, Activity>,
    activity_inputs_cache: BoscaCache<String, Vec<ActivityParameter>>,
    activity_outputs_cache: BoscaCache<String, Vec<ActivityParameter>>,

    workflow_cache: BoscaCache<String, Workflow>,
    workflow_activity_ids_cache: BoscaCache<String, Vec<i64>>,
    workflow_activity_cache: BoscaCache<i64, WorkflowActivity>,
    workflow_activity_inputs_cache: BoscaCache<i64, Vec<WorkflowActivityParameter>>,
    workflow_activity_outputs_cache: BoscaCache<i64, Vec<WorkflowActivityParameter>>,
    workflow_activity_models_cache: BoscaCache<i64, Vec<WorkflowActivityModel>>,
    workflow_activity_prompts_cache: BoscaCache<i64, Vec<WorkflowActivityPrompt>>,
    workflow_activity_storage_systems_cache: BoscaCache<i64, Vec<WorkflowActivityStorageSystem>>,
}

impl WorkflowCache {
    pub async fn new(cache: &mut BoscaCacheManager) -> Self {
        Self {
            trait_cache: cache.new_string_tiered_cache("traits", 5000, TieredCacheType::Trait).await,
            trait_workflow_ids_cache: cache.new_string_tiered_cache(
                "trait_workflow_ids",
                5000,
                TieredCacheType::Trait,
            ).await,
            workflow_cache: cache.new_string_tiered_cache(
                "workflows",
                5000,
                TieredCacheType::Workflow,
            ).await,
            storage_system_cache: cache.new_id_tiered_cache(
                "storage_systems",
                5000,
                TieredCacheType::StorageSystem,
            ).await,
            storage_system_models_cache: cache.new_id_tiered_cache(
                "storage_system_models",
                5000,
                TieredCacheType::StorageSystem,
            ).await,
            prompt_cache: cache.new_id_tiered_cache("prompts", 5000, TieredCacheType::Prompt).await,
            model_cache: cache.new_id_tiered_cache("models", 5000, TieredCacheType::Model).await,
            state_cache: cache.new_string_tiered_cache("states", 100, TieredCacheType::State).await,
            transition_cache: cache.new_string_tiered_cache(
                "transitions",
                100,
                TieredCacheType::Transition,
            ).await,
            activity_cache: cache.new_string_tiered_cache(
                "activities",
                5000,
                TieredCacheType::Activity,
            ).await,
            activity_inputs_cache: cache.new_string_tiered_cache(
                "activity_inputs",
                5000,
                TieredCacheType::Activity,
            ).await,
            activity_outputs_cache: cache.new_string_tiered_cache(
                "activity_outputs",
                5000,
                TieredCacheType::Activity,
            ).await,
            workflow_activity_ids_cache: cache.new_string_tiered_cache(
                "workflow_activity_ids",
                5000,
                TieredCacheType::Workflow,
            ).await,
            workflow_activity_cache: cache.new_int_tiered_cache(
                "workflow_activities",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
            workflow_activity_inputs_cache: cache.new_int_tiered_cache(
                "workflow_activity_inputs",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
            workflow_activity_outputs_cache: cache.new_int_tiered_cache(
                "workflow_activity_outputs",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
            workflow_activity_models_cache: cache.new_int_tiered_cache(
                "workflow_activity_models",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
            workflow_activity_prompts_cache: cache.new_int_tiered_cache(
                "workflow_activity_prompts",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
            workflow_activity_storage_systems_cache: cache.new_int_tiered_cache(
                "workflow_activity_storage_systems",
                5000,
                TieredCacheType::WorkflowActivity,
            ).await,
        }
    }

    pub async fn get_transition(&self, from_state_id: &String, to_state_id: &String) -> Option<Transition> {
        let key = format!("{}-{}", from_state_id, to_state_id);
        self.transition_cache.get(&key).await
    }

    pub async fn set_transition(&self, transition: &Transition) {
        let key = format!("{}-{}", transition.from_state_id, transition.to_state_id);
        self.transition_cache.set(&key, transition).await;
    }

    pub async fn evict_transition(&self, from_state_id: &String, to_state_id: &String) {
        let key = format!("{}-{}", from_state_id, to_state_id);
        self.transition_cache.remove(&key).await;
    }

    pub async fn get_trait(&self, id: &String) -> Option<Trait> {
        self.trait_cache.get(id).await
    }

    pub async fn set_trait(&self, t: &Trait) {
        self.trait_cache.set(&t.id, t).await;
    }

    pub async fn evict_trait(&self, trait_id: &String) {
        self.trait_cache.remove(trait_id).await;
    }

    pub async fn get_prompt(&self, id: &Uuid) -> Option<Prompt> {
        self.prompt_cache.get(id).await
    }

    pub async fn set_prompt(&self, prompt: &Prompt) {
        self.prompt_cache.set(&prompt.id, prompt).await;
    }

    pub async fn evict_prompt(&self, prompt_id: &Uuid) {
        self.prompt_cache.remove(prompt_id).await;
    }

    pub async fn get_model(&self, id: &Uuid) -> Option<Model> {
        self.model_cache.get(id).await
    }

    pub async fn set_model(&self, model: &Model) {
        self.model_cache.set(&model.id, model).await;
    }

    pub async fn evict_model(&self, model_id: &Uuid) {
        self.model_cache.remove(model_id).await;
    }

    pub async fn get_state(&self, id: &String) -> Option<WorkflowState> {
        self.state_cache.get(id).await
    }

    pub async fn set_state(&self, state: &WorkflowState) {
        self.state_cache.set(&state.id, state).await;
    }

    pub async fn evict_state(&self, state_id: &String) {
        self.state_cache.remove(state_id).await;
    }

    pub async fn get_storage_system(&self, id: &Uuid) -> Option<StorageSystem> {
        self.storage_system_cache.get(id).await
    }

    pub async fn set_storage_system(&self, system: &StorageSystem) {
        self.storage_system_cache.set(&system.id, system).await;
    }

    pub async fn get_storage_system_models(&self, id: &Uuid) -> Option<Vec<StorageSystemModel>> {
        self.storage_system_models_cache.get(id).await
    }

    pub async fn set_storage_system_models(&self, id: &Uuid, models: &Vec<StorageSystemModel>) {
        self.storage_system_models_cache.set(id, models).await;
    }

    pub async fn evict_storage_system(&self, system_id: &Uuid) {
        self.storage_system_cache.remove(system_id).await;
        self.storage_system_models_cache.remove(system_id).await;
    }

    pub async fn get_workflow(&self, id: &String) -> Option<Workflow> {
        self.workflow_cache.get(id).await
    }

    pub async fn set_workflow(&self, workflow: &Workflow) {
        self.workflow_cache.set(&workflow.id, workflow).await;
    }

    pub async fn get_workflow_activity(&self, id: &i64) -> Option<WorkflowActivity> {
        self.workflow_activity_cache.get(id).await
    }

    pub async fn set_workflow_activity(&self, activity: &WorkflowActivity) {
        self.workflow_activity_cache
            .set(&activity.id, activity)
            .await;
    }

    pub async fn evict_workflow(&self, workflow_id: &String) {
        if let Some(activity_ids) = self.get_workflow_activity_ids(workflow_id).await {
            for id in activity_ids {
                self.evict_workflow_activity(&id).await;
            }
        }
        self.workflow_activity_ids_cache.remove(workflow_id).await;
        self.workflow_cache.remove(workflow_id).await;
    }

    pub async fn evict_workflow_activity(&self, id: &i64) {
        self.workflow_activity_cache.remove(id).await;
        self.workflow_activity_inputs_cache.remove(id).await;
        self.workflow_activity_outputs_cache.remove(id).await;
        self.workflow_activity_models_cache.remove(id).await;
        self.workflow_activity_prompts_cache.remove(id).await;
        self.workflow_activity_storage_systems_cache
            .remove(id)
            .await;
    }

    pub async fn get_trait_workflow_ids(&self, key: &String) -> Option<Vec<String>> {
        self.trait_workflow_ids_cache.get(key).await
    }

    pub async fn set_trait_workflow_ids(&self, key: &String, value: &Vec<String>) {
        self.trait_workflow_ids_cache.set(key, value).await;
    }

    pub async fn get_workflow_activity_ids(&self, workflow_id: &String) -> Option<Vec<i64>> {
        self.workflow_activity_ids_cache.get(workflow_id).await
    }

    pub async fn set_workflow_activity_ids(&self, workflow_id: &String, ids: &Vec<i64>) {
        self.workflow_activity_ids_cache.set(workflow_id, ids).await;
    }

    pub async fn get_workflow_activity_inputs(
        &self,
        id: &i64,
    ) -> Option<Vec<WorkflowActivityParameter>> {
        self.workflow_activity_inputs_cache.get(id).await
    }

    pub async fn set_workflow_activity_inputs(
        &self,
        id: &i64,
        parameters: &Vec<WorkflowActivityParameter>,
    ) {
        self.workflow_activity_inputs_cache
            .set(id, parameters)
            .await
    }

    pub async fn get_workflow_activity_outputs(
        &self,
        id: &i64,
    ) -> Option<Vec<WorkflowActivityParameter>> {
        self.workflow_activity_outputs_cache.get(id).await
    }

    pub async fn set_workflow_activity_outputs(
        &self,
        id: &i64,
        parameters: &Vec<WorkflowActivityParameter>,
    ) {
        self.workflow_activity_outputs_cache
            .set(id, parameters)
            .await
    }

    pub async fn get_workflow_activity_models(
        &self,
        id: &i64,
    ) -> Option<Vec<WorkflowActivityModel>> {
        self.workflow_activity_models_cache.get(id).await
    }

    pub async fn set_workflow_activity_models(
        &self,
        id: &i64,
        models: &Vec<WorkflowActivityModel>,
    ) {
        self.workflow_activity_models_cache.set(id, models).await
    }

    pub async fn get_workflow_activity_prompts(
        &self,
        id: &i64,
    ) -> Option<Vec<WorkflowActivityPrompt>> {
        self.workflow_activity_prompts_cache.get(id).await
    }

    pub async fn set_workflow_activity_prompts(
        &self,
        id: &i64,
        prompts: &Vec<WorkflowActivityPrompt>,
    ) {
        self.workflow_activity_prompts_cache.set(id, prompts).await
    }

    pub async fn get_workflow_activity_storage_systems(
        &self,
        id: &i64,
    ) -> Option<Vec<WorkflowActivityStorageSystem>> {
        self.workflow_activity_storage_systems_cache.get(id).await
    }

    pub async fn set_workflow_activity_storage_systems(
        &self,
        id: &i64,
        systems: &Vec<WorkflowActivityStorageSystem>,
    ) {
        self.workflow_activity_storage_systems_cache
            .set(id, systems)
            .await
    }

    pub async fn get_activity(&self, activity_id: &String) -> Option<Activity> {
        self.activity_cache.get(activity_id).await
    }

    pub async fn set_activity(&self, activity: &Activity) {
        self.activity_cache.set(&activity.id, activity).await;
    }

    pub async fn get_activity_inputs(
        &self,
        activity_id: &String,
    ) -> Option<Vec<ActivityParameter>> {
        self.activity_inputs_cache.get(activity_id).await
    }

    pub async fn set_activity_inputs(&self, id: &String, activities: &Vec<ActivityParameter>) {
        self.activity_inputs_cache.set(id, activities).await;
    }

    pub async fn get_activity_outputs(
        &self,
        activity_id: &String,
    ) -> Option<Vec<ActivityParameter>> {
        self.activity_outputs_cache.get(activity_id).await
    }

    pub async fn set_activity_outputs(&self, id: &String, activities: &Vec<ActivityParameter>) {
        self.activity_outputs_cache.set(id, activities).await;
    }

    pub async fn evict_activity(&self, activity_id: &String) {
        self.activity_cache.remove(activity_id).await;
        self.activity_inputs_cache.remove(activity_id).await;
        self.activity_outputs_cache.remove(activity_id).await;
    }
}
