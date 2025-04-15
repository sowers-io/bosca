#[derive(Clone)]
pub enum TieredCacheType {
    Slug,
    Metadata,
    Collection,
    State,
    Transition,
    StorageSystem,
    Model,
    Prompt,
    Workflow,
    Trait,
    Activity,
    WorkflowActivity,
    Principal,
}
