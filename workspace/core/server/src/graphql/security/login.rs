use std::fmt::{Debug, Formatter};
use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::graphql::security::principal::PrincipalObject;
use crate::graphql::security::token::TokenObject;
use crate::models::profiles::profile::Profile;
use crate::models::security::principal::Principal;
use crate::security::token::Token;
use async_graphql::*;

pub struct LoginObject {}

pub struct LoginResponse {
    pub profile: Option<Profile>,
    pub principal: Principal,
    pub token: Token,
    pub refresh_token: String,
}

impl Debug for LoginResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoginResponse").finish()
    }
}

#[Object]
impl LoginResponse {
    async fn profile(&self) -> Option<ProfileObject> {
        self.profile.clone().map(ProfileObject::new)
    }

    async fn principal(&self) -> PrincipalObject {
        PrincipalObject::new(self.principal.clone())
    }

    async fn token(&self) -> TokenObject<'_> {
        TokenObject::new(&self.token)
    }

    async fn refresh_token(&self) -> &String {
        &self.refresh_token
    }
}

#[Object(name = "Login")]
impl LoginObject {

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
}
