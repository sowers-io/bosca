use crate::graphql::security::principals::PrincipalObject;
use crate::graphql::security::token::TokenObject;
use crate::models::security::principal::Principal;
use crate::security::token::Token;
use async_graphql::*;
use crate::context::BoscaContext;
use crate::graphql::profiles::profile::ProfileObject;
use crate::models::profile::profile::Profile;

pub struct LoginObject {}

pub struct LoginResponse {
    pub profile: Option<Profile>,
    pub principal: Principal,
    pub token: Token,
}

#[Object]
impl LoginResponse {
    async fn profile(&self) -> Option<ProfileObject> {
        self.profile.clone().map(ProfileObject::new)
    }

    async fn principal(&self) -> PrincipalObject {
        PrincipalObject::new(self.principal.clone())
    }

    async fn token(&self) -> TokenObject {
        TokenObject::new(&self.token)
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
