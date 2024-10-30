use async_graphql::{Context, Error};
use crate::context::BoscaContext;

pub async fn check_has_group(ctx: &Context<'_>, group: &str) -> Result<(), Error> {
    let ctx = ctx.data::<BoscaContext>()?;
    let g = group.to_string();
    let group = ctx.security.get_group_by_name(&g).await?;
    if !ctx.principal.has_group(&group.id) {
        let admin = ctx.security.get_administrators_group().await?;
        if !ctx.principal.has_group(&admin.id) {
            return Err(Error::new("invalid permissions"));
        }
    }
    Ok(())
}
