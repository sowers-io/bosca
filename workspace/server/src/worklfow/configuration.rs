use crate::models::workflow::activities::{ActivityInput, WorkflowActivityInput};
use crate::models::workflow::models::ModelInput;
use crate::models::workflow::prompts::PromptInput;
use crate::models::workflow::states::WorkflowStateInput;
use crate::models::workflow::storage_system_models::StorageSystemModelInput;
use crate::models::workflow::storage_systems::StorageSystemInput;
use crate::models::workflow::traits::TraitInput;
use crate::models::workflow::transitions::TransitionInput;
use crate::models::workflow::workflows::WorkflowInput;
use std::collections::HashMap;
use uuid::Uuid;
use yaml_rust2::Yaml;
use crate::datastores::workflow::WorkflowDataStore;
use crate::worklfow::yaml::into;

pub async fn configure(yaml: &Yaml, datasource: &WorkflowDataStore) -> bool {
    let mut model_ids: HashMap<String, Uuid> = HashMap::new();
    let mut storage_system_ids: HashMap<String, Uuid> = HashMap::new();
    let mut prompt_ids: HashMap<String, Uuid> = HashMap::new();
    {
        let hash = yaml["models"].as_hash().unwrap();
        for key in hash.keys() {
            let mi: ModelInput = hash.get(key).unwrap().into();
            let id = datasource.add_model(&mi).await.unwrap();
            model_ids.insert(key.as_str().unwrap().to_string(), id);
        }
    }
    {
        let hash = yaml["storageSystems"].as_hash().unwrap();
        for key in hash.keys() {
            let s = hash.get(key).unwrap();
            let mi: StorageSystemInput = s.into();
            let id = datasource.add_storage_system(&mi).await.unwrap();
            storage_system_ids.insert(key.as_str().unwrap().to_string(), id);
            if s["models"].is_null() || s["models"].is_badvalue() {
                continue;
            }
            let m = s["models"].as_hash().unwrap();
            for key in m.keys() {
                let m = m.get(key).unwrap();
                let mut sm: StorageSystemModelInput = m.into();
                sm.model_id = model_ids.get(key.as_str().unwrap()).unwrap().to_string();
                datasource.add_storage_system_model(&id, &sm).await.unwrap();
            }
        }
    }
    {
        let hash = yaml["prompts"].as_hash().unwrap();
        for key in hash.keys() {
            let mi: PromptInput = hash.get(key).unwrap().into();
            let id = datasource.add_prompt(&mi).await.unwrap();
            prompt_ids.insert(key.as_str().unwrap().to_string(), id);
        }
    }
    {
        let workflows = &yaml["workflows"];
        {
            let hash = workflows["activities"].as_hash().unwrap();
            for key in hash.keys() {
                let mut ai: ActivityInput = hash.get(key).unwrap().into();
                ai.id = key.as_str().unwrap().to_string();
                datasource.add_activity(&ai).await.unwrap();
            }
        }
        {
            let workflows_hash = workflows["workflows"].as_hash().unwrap();
            for key in workflows_hash.keys() {
                let workflow = workflows_hash.get(key).unwrap();
                let workflow_id = key.as_str().unwrap().to_string();
                let queue = workflow["queue"].as_str().unwrap().to_string();
                let activities_hash = workflow["activities"].as_hash().unwrap();
                let mut activities = Vec::new();
                for key in activities_hash.keys() {
                    let w = activities_hash.get(key).unwrap();
                    let mut a: WorkflowActivityInput = w.into();
                    a.activity_id = key.as_str().unwrap().to_string();
                    if a.queue.is_empty() {
                        a.queue = queue.clone();
                    }
                    a.models = a
                        .models
                        .into_iter()
                        .map(|mut p| {
                            p.model_id = model_ids.get(&p.model_id).unwrap().to_string();
                            p
                        })
                        .collect();
                    a.prompts = a
                        .prompts
                        .into_iter()
                        .map(|mut p| {
                            p.prompt_id = prompt_ids.get(&p.prompt_id).unwrap().to_string();
                            p
                        })
                        .collect();
                    a.storage_systems = a
                        .storage_systems
                        .into_iter()
                        .map(|mut p| {
                            p.system_id = storage_system_ids.get(&p.system_id).unwrap().to_string();
                            p
                        })
                        .collect();
                    activities.push(a);
                }
                let workflow = WorkflowInput {
                    id: workflow_id,
                    name: workflow["name"].as_str().unwrap_or("").to_string(),
                    queue: if !queue.is_empty() { queue.clone() } else { "default".to_owned() },
                    description: workflow["description"]
                        .as_str()
                        .unwrap_or("")
                        .to_string(),
                    configuration: into(&workflow["configuration"]),
                    activities
                };
                datasource.add_workflow(&workflow).await.unwrap();
            }
        }
        {
            let hash = workflows["states"].as_hash().unwrap();
            for key in hash.keys() {
                let mut si: WorkflowStateInput = hash.get(key).unwrap().into();
                si.id = key.as_str().unwrap().to_string();
                datasource.add_state(&si).await.unwrap();
            }
        }
        {
            let transitions = workflows["transitions"].as_vec().unwrap();
            for t in transitions.iter() {
                let ti: TransitionInput = t.into();
                datasource.add_transition(&ti).await.unwrap();
            }
        }
    }
    {
        let hash = yaml["traits"].as_hash().unwrap();
        for key in hash.keys() {
            let mut ti: TraitInput = hash.get(key).unwrap().into();
            ti.id = key.as_str().unwrap().to_string();
            datasource.add_trait(&ti).await.unwrap();
        }
    }
    datasource.create_queues().await.unwrap();
    true
}
