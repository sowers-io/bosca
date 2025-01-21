use crate::graphql::workflows::workflow::WorkflowObject;
use crate::models::workflow::traits::Trait;
use async_graphql::{Context, Error, Object};
use crate::context::BoscaContext;

pub struct TraitObject {
    trait_: Trait,
}

impl TraitObject {
    pub fn new(trait_: Trait) -> Self {
        Self { trait_ }
    }
}

#[Object(name = "Trait")]
impl TraitObject {
    async fn id(&self) -> String {
        self.trait_.id.to_string()
    }

    async fn name(&self) -> &String {
        &self.trait_.name
    }

    async fn description(&self) -> &String {
        &self.trait_.description
    }

    async fn content_types(&self) -> &Vec<String> {
        &self.trait_.content_types
    }

    async fn workflow_ids(&self) -> &Vec<String> {
        &self.trait_.workflow_ids
    }

    async fn workflows(&self, ctx: &Context<'_>) -> Result<Vec<WorkflowObject>, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let workflows = ctx.workflow.get_workflows_by_trait(&self.trait_.id).await?;
        Ok(workflows.into_iter().map(WorkflowObject::new).collect())
    }
}

impl From<Trait> for TraitObject {
    fn from(trait_: Trait) -> Self {
        Self::new(trait_)
    }
}
