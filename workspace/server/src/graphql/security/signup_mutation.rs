use crate::context::BoscaContext;
use crate::models::profile::profile::ProfileInput;
use crate::models::security::credentials::PasswordCredential;
use async_graphql::*;
use crate::graphql::security::principals::PrincipalObject;

pub struct SignupMutationObject {}

#[Object(name = "SignupMutation")]
impl SignupMutationObject {
    async fn password(
        &self,
        ctx: &Context<'_>,
        identifier: String,
        password: String,
        profile: ProfileInput,
    ) -> Result<PrincipalObject, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let password_credential = PasswordCredential::new(identifier, password);
        let groups = vec![];
        let principal_id = ctx
            .security
            .add_principal(
                false,
                serde_json::Value::Null,
                &password_credential,
                &groups,
            )
            .await?;
        ctx.profile.add_profile(&principal_id, &profile).await?;
        let principal = ctx.security
            .get_principal_by_id(&principal_id)
            .await?;
        // TODO: Send Verification Email

        println!("{:?}", principal);

        Ok(PrincipalObject::new(principal))
    }

    async fn password_verify(&self, ctx: &Context<'_>, verification_token: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.security.set_principal_verified(&verification_token).await?;
        Ok(true)
    }
}
