use crate::context::BoscaContext;
use crate::graphql::security::login::LoginResponse;
use async_graphql::*;

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
        let principal = ctx
            .security
            .get_principal_by_password(identifier.as_str(), password.as_str())
            .await?;
        if !principal.verified {
            return Err(Error::new("not verified"));
        }
        let profile = ctx.profile.get_by_principal(&principal.id).await?;
        let token = ctx.security.new_token(&principal)?;
        let refresh_token = ctx.security.new_refresh_token(&principal)?;
        ctx.security.add_refresh_token(&principal, &refresh_token).await?;
        Ok(LoginResponse {
            refresh_token: refresh_token.token,
            profile,
            principal,
            token,
        })
    }

    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> Result<LoginResponse, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal = ctx.security.validate_refresh_token(&refresh_token).await?;
        if let Some(principal_id) = principal {
            let principal = ctx.security.get_principal_by_id(&principal_id).await?;
            if !principal.verified {
                return Err(Error::new("not verified"));
            }
            let profile = ctx.profile.get_by_principal(&principal.id).await?;
            let token = ctx.security.new_token(&principal)?;
            let refresh_token = ctx.security.new_refresh_token(&principal)?;
            ctx.security.add_refresh_token(&principal, &refresh_token).await?;
            Ok(LoginResponse {
                refresh_token: refresh_token.token,
                profile,
                principal,
                token,
            })
        } else {
            Err(Error::new("invalid refresh token"))
        }
    }
}
