use crate::context::BoscaContext;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::graphql::security::login::LoginResponse;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::{PROFILE_FORGOTPASSWORD, SEND_EMAIL};
use async_graphql::*;
use serde_json::json;

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
        ctx.security
            .add_refresh_token(&principal, &refresh_token)
            .await?;
        Ok(LoginResponse {
            refresh_token: refresh_token.token,
            profile,
            principal,
            token,
        })
    }

    async fn refresh_token(
        &self,
        ctx: &Context<'_>,
        refresh_token: String,
    ) -> Result<LoginResponse, Error> {
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
            ctx.security
                .add_refresh_token(&principal, &refresh_token)
                .await?;
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

    async fn forgot_password(&self, ctx: &Context<'_>, identifier: String) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal = ctx
            .security
            .get_principal_by_identifier(&identifier)
            .await?;
        if !principal.verified {
            return Err(Error::new("not verified"));
        }
        let Some(profile) = ctx.profile.get_by_principal(&principal.id).await? else {
            return Ok(false);
        };
        let verification_token = ctx
            .security
            .create_verification_token(&principal.id)
            .await?;
        let mut request = EnqueueRequest {
            workflow_id: Some(PROFILE_FORGOTPASSWORD.to_string()),
            profile_id: Some(profile.id),
            configurations: Some(vec![WorkflowConfigurationInput {
                activity_id: SEND_EMAIL.to_string(),
                configuration: json!({
                    "attributes": {
                        "verification_token": verification_token
                    }
                }),
            }]),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(true)
    }
}
