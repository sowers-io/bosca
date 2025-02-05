use async_graphql::*;
use crate::context::BoscaContext;
use crate::graphql::security::login::LoginResponse;

pub struct LoginMutationObject {}

#[Object(name = "LoginMutation")]
impl LoginMutationObject {
    async fn password(
        &self,
        ctx: &Context<'_>,
        identifier: String,
        password: String,
    ) -> Result<LoginResponse, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal = ctx.security
            .get_principal_by_password(identifier.as_str(), password.as_str())
            .await?;
        if !principal.verified {
            return Err(Error::new("not verified"));
        }
        let profile = ctx.profile
            .get_profile_by_principal(&principal.id)
            .await?;
        let token = ctx.security.new_token(&principal)?;
        Ok(LoginResponse { profile, principal, token })
    }
}
