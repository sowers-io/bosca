use crate::graphql::security::principals::PrincipalObject;
use async_graphql::*;
use uuid::Uuid;
use crate::context::BoscaContext;
use crate::models::security::credentials::PasswordCredential;

pub struct SecurityMutationObject {}

#[Object(name = "SecurityMutation")]
impl SecurityMutationObject {

    async fn signup(
        &self,
        ctx: &Context<'_>,
        identifier: String,
        password: String,
    ) -> Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let attributes = serde_json::Value::Null;
        let credentials = PasswordCredential::new(identifier, password);
        let groups = vec![];
        let principal_id = ctx.security.add_principal(
            false,
            attributes,
            &credentials,
            &groups,
        ).await?;
        let principal = ctx.security
            .get_principal_by_id(&principal_id)
            .await?;
        Ok(PrincipalObject::new(principal))
    }

    async fn add_principal_group(
        &self,
        ctx: &Context<'_>,
        principal_id: String,
        group_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let admin_group = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin_group.id) {
            return Err(Error::new("invalid permissions"));
        }
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.add_principal_group(&id, &group_id).await?;
        Ok(true)
    }
}
