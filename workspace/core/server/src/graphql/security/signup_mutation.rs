use crate::context::BoscaContext;
use crate::graphql::security::principal::PrincipalObject;
use crate::models::profiles::profile::ProfileInput;
use crate::util::profile::add_password_principal;
use async_graphql::*;

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
        let principal = add_password_principal(
            ctx,
            &identifier,
            &password,
            &profile,
            false,
            true
        )
        .await?;

        // TODO: Send Verification Email
        println!("{:?}", principal);

        Ok(PrincipalObject::new(principal))
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
