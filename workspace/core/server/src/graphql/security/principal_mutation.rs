use crate::context::BoscaContext;
use crate::models::security::credentials::CredentialType;
use crate::models::security::principal::Principal;
use async_graphql::{Context, Error, Object};

pub struct PrincipalMutation {
    principal: Principal,
}

impl PrincipalMutation {
    pub fn new(principal: Principal) -> Self {
        Self { principal }
    }
}

#[Object(name = "PrincipalMutation")]
impl PrincipalMutation {
    async fn identifier(
        &self,
        ctx: &Context<'_>,
        identifier: String,
    ) -> async_graphql::Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous || !ctx.principal.verified {
            return Err(Error::new("unauthorized"));
        }
        let mut password_credential = {
            let credentials = ctx
                .security
                .get_principal_credentials(&self.principal.id)
                .await?;
            let Some(credential) = credentials
                .into_iter()
                .filter(|c| c.get_type() == CredentialType::Password)
                .next()
            else {
                return Err(Error::new("invalid principal"));
            };
            credential
        };
        password_credential.set_identifier(identifier);
        ctx.security
            .set_principal_credential(&self.principal.id, &password_credential)
            .await?;
        Ok(true)
    }

    async fn password(
        &self,
        ctx: &Context<'_>,
        identifier: Option<String>,
        old_password: String,
        new_password: String,
    ) -> async_graphql::Result<bool, Error> {
        let ctx = ctx.data::<BoscaContext>()?;
        if ctx.principal.anonymous || !ctx.principal.verified {
            return Err(Error::new("unauthorized"));
        }
        let credentials = ctx
            .security
            .get_principal_credentials(&self.principal.id)
            .await?;
        let Some(mut credential) = credentials
            .into_iter()
            .filter(|c| c.get_type() == CredentialType::Password)
            .next()
        else {
            return Err(Error::new("invalid principal"));
        };
        if !ctx
            .security
            .verify_password(&credential, old_password.as_str())?
        {
            return Err(Error::new("invalid password"));
        }
        if let Some(identifier) = identifier {
            credential.set_identifier(identifier);
        }
        credential.set_password(new_password)?;
        ctx.security
            .set_principal_credential(&self.principal.id, &credential)
            .await?;
        Ok(true)
    }
}
