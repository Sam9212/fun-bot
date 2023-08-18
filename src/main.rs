mod commands;
mod utils;

use dotenv::dotenv;
use commands::example::example;
use poise::{
    serenity_prelude as serenity,
    FrameworkError::{Setup, Command}, FrameworkContext,
};

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {
    /*
        Useful global state goes here.
        Data struct can be accessed
        from every command.
     */
}

async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        Setup { error, .. } => panic!("failed to start bot: {:?}", error),
        Command { error, ctx } => println!("Command `{}` emitted an error: {:?}", ctx.command().name, error),
        _ => {},
    }
}

async fn event_handler<'a, U, E>(
    _ctx: &serenity::Context,
    event: &poise::Event<'a>,
    _framework: FrameworkContext<'a, U, E>,
) -> Result<(), Error> {
    println!("event: {}", event.name());
    Ok(())
}

#[tokio::main]
async fn main() {
    // Getting token from .env
    dotenv().ok();

    // Configuring bot options
    let options = poise::FrameworkOptions {
        commands: vec![
            example(),
        ],
        prefix_options: poise::PrefixFrameworkOptions {
            prefix: Some("g.".into()),
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, FrameworkContext, _| Box::pin(async move { event_handler(ctx, event, FrameworkContext).await }),
        ..Default::default()
    };

    poise::Framework::builder()
        .token(
            std::env::var("TOKEN").expect("Token Missing From .env!"),
        )
        .setup(|_ctx, ready, _framework| {
            Box::pin(async move {
                println!("Bot logged in as `{}`", ready.user.name);
                Ok(Data {})
            })
        })
        .options(options)
        .intents(serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT)
        .run().await.unwrap();
}