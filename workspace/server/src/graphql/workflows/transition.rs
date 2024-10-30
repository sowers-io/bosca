use crate::models::workflow::transitions::Transition;
use async_graphql::Object;

pub struct TransitionObject {
    transition: Transition,
}

impl TransitionObject {
    pub fn new(transition: Transition) -> Self {
        Self { transition }
    }
}

#[Object(name = "Transition")]
impl TransitionObject {
    #[allow(clippy::wrong_self_convention)]
    async fn from_state_id(&self) -> &String {
        &self.transition.from_state_id
    }

    async fn to_state_id(&self) -> &String {
        &self.transition.to_state_id
    }

    async fn description(&self) -> &String {
        &self.transition.description
    }
}

impl From<Transition> for TransitionObject {
    fn from(transition: Transition) -> Self {
        Self::new(transition)
    }
}
