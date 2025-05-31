use crate::context::BoscaContext;
use crate::graphql::content::metadata_mutation::WorkflowConfigurationInput;
use crate::graphql::security::principal::PrincipalObject;
use crate::models::profiles::profile::ProfileInput;
use crate::models::security::credentials::Credential;
use crate::models::security::credentials_password::PasswordCredential;
use crate::models::workflow::enqueue_request::EnqueueRequest;
use crate::workflow::core_workflow_ids::{PROFILE_SIGNUP, PROFILE_UPDATE_STORAGE, SEND_EMAIL};
use async_graphql::*;
use serde_json::json;

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

        let password_credential = Credential::Password(PasswordCredential::new(
            identifier.to_string(),
            password.to_string(),
        )?);

        let (principal_id, profile_id) = ctx.security.add_principal_with_credential(
            ctx,
            &password_credential,
            &profile,
            None,
            true,
            true,
        ).await?;

        let principal = ctx.security.get_principal_by_id(&principal_id).await?;

        if !ctx.security.auto_verify_accounts {
            let mut request = EnqueueRequest {
                workflow_id: Some(PROFILE_SIGNUP.to_string()),
                profile_id: Some(profile_id),
                configurations: Some(vec![WorkflowConfigurationInput {
                    activity_id: SEND_EMAIL.to_string(),
                    configuration: json!({
                        "attributes": {
                            "verification_token": principal.verification_token.clone().unwrap()
                        }
                    }),
                }]),
                ..Default::default()
            };
            ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        } else {
            let mut request = EnqueueRequest {
                workflow_id: Some(PROFILE_UPDATE_STORAGE.to_string()),
                profile_id: Some(profile_id),
                ..Default::default()
            };
            ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        }

        // TODO: welcome email

        Ok(PrincipalObject::new(principal))
    }

    async fn resend_password_verification(
        &self,
        ctx: &Context<'_>,
        identifier: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal = ctx
            .security
            .get_principal_by_identifier(&identifier)
            .await?;
        if !principal.verified {
            return Ok(false);
        }
        let Some(profile) = ctx.profile.get_by_principal(&principal.id).await? else {
            return Ok(false);
        };
        let mut request = EnqueueRequest {
            workflow_id: Some(PROFILE_SIGNUP.to_string()),
            profile_id: Some(profile.id),
            configurations: Some(vec![WorkflowConfigurationInput {
                activity_id: SEND_EMAIL.to_string(),
                configuration: json!({
                    "attributes": {
                        "verification_token": principal.verification_token.clone().unwrap()
                    }
                }),
            }]),
            ..Default::default()
        };
        ctx.workflow.enqueue_workflow(ctx, &mut request).await?;
        Ok(true)
    }

    async fn password_verify(
        &self,
        ctx: &Context<'_>,
        verification_token: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.security
            .set_principal_verified(&verification_token)
            .await?;
        Ok(true)
    }
}
