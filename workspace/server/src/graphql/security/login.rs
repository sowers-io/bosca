use crate::graphql::security::principals::PrincipalObject;
use crate::graphql::security::token::TokenObject;
use crate::models::security::principal::Principal;
use crate::security::token::Token;
use async_graphql::*;
use crate::context::BoscaContext;

pub struct LoginObject {}

pub struct LoginResponse {
    principal: Principal,
    token: Token,
}

#[Object]
impl LoginResponse {
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
        let token = ctx.security.new_token(&principal)?;
        Ok(LoginResponse { principal, token })
    }
}
