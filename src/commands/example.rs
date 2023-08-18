use crate::{Context, Error};

#[poise::command(prefix_command)]
pub async fn example(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("pong").await?;
    Ok(())
}