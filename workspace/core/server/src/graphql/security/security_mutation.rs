use crate::context::BoscaContext;
use crate::graphql::security::groups_mutation::GroupsMutation;
use crate::graphql::security::login_mutation::LoginMutationObject;
use crate::graphql::security::principal_mutation::PrincipalMutation;
use crate::graphql::security::security_firebase_mutation::SecurityFirebaseMutationObject;
use crate::graphql::security::signup_mutation::SignupMutationObject;
use crate::models::security::credentials::{Credential, CredentialType};
use crate::models::security::credentials_password::PasswordCredential;
use async_graphql::*;
use uuid::Uuid;

pub struct SecurityMutationObject {}

#[Object(name = "SecurityMutation")]
impl SecurityMutationObject {
    async fn login(&self) -> LoginMutationObject {
        LoginMutationObject {}
    }

    async fn signup(&self) -> SignupMutationObject {
        SignupMutationObject {}
    }

    async fn groups(&self) -> GroupsMutation {
        GroupsMutation {}
    }

    async fn principal(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
    ) -> Result<PrincipalMutation, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous {
            return Err(Error::new("unauthenticated"));
        }
        if !ctx.principal.verified {
            return Err(Error::new("not verified"));
        }
        let principal = if let Some(id) = id {
            let id = Uuid::parse_str(&id)?;
            ctx.check_has_service_account().await?;
            ctx.security.get_principal_by_id(&id).await?
        } else {
            ctx.principal.clone()
        };
        Ok(PrincipalMutation::new(principal))
    }

    async fn password(
        &self,
        ctx: &Context<'_>,
        verification_token: String,
        new_password: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        let principal = ctx
            .security
            .verify_verification_token(&verification_token)
            .await?;
        let credentials = ctx
            .security
            .get_principal_credentials(&principal.id)
            .await?;
        if let Some(credential) = credentials
            .iter()
            .find(|c| c.get_type() == CredentialType::Password)
        {
            let mut credential = credential.clone();
            credential.set_password(new_password)?;
            ctx.security
                .set_principal_credential(&principal.id, &credential)
                .await?;
        } else {
            let Some(profile) = ctx.profile.get_by_principal(&principal.id).await? else {
                return Err(Error::new("missing profile"));
            };
            let attributes = ctx.profile.get_attributes(&profile.id).await?;
            let Some(email_attr) = attributes
                .iter()
                .find(|a| a.type_id == "bosca.profiles.email")
            else {
                return Err(Error::new("missing email"));
            };
            let email = email_attr
                .attributes
                .as_ref()
                .expect("missing email attributes")
                .get("email")
                .expect("missing email value")
                .as_str()
                .expect("failed to get email value")
                .to_string();
            let credential = Credential::Password(PasswordCredential::new(email, new_password)?);
            ctx.security
                .add_principal_credential(&principal.id, &credential)
                .await?;
        }

        ctx.security
            .set_principal_verified(&verification_token)
            .await?;
        // KJB: TODO: Send change email
        Ok(true)
    }

    async fn expire_refresh_tokens(&self, ctx: &Context<'_>) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.security.expire_refresh_tokens().await?;
        Ok(true)
    }

    async fn add_principal_group(
        &self,
        ctx: &Context<'_>,
        principal_id: String,
        group_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.add_principal_group(&id, &group_id).await?;
        Ok(true)
    }

    async fn remove_principal_group(
        &self,
        ctx: &Context<'_>,
        principal_id: String,
        group_id: String,
    ) -> Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        ctx.check_has_admin_account().await?;
        let id = Uuid::parse_str(principal_id.as_str())?;
        let group_id = Uuid::parse_str(group_id.as_str())?;
        ctx.security.remove_principal_group(&id, &group_id).await?;
        Ok(true)
    }

    async fn firebase(&self) -> SecurityFirebaseMutationObject {
        SecurityFirebaseMutationObject {}
    }
}
