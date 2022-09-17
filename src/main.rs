use joke::send_joke;
use moderation::kick;
use serenity::async_trait;
use serenity::model::Permissions;
use serenity::model::application::interaction::{InteractionResponseType, Interaction};
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::*;
use dotenv::dotenv;
use std::env;


mod joke;
mod moderation;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        let guild = GuildId(env::var("GUILD_ID").expect("Expected GUILD ID in env file").parse().expect("d"));

        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hello, I am daddy!".to_string(),
                "joke" => send_joke().await.unwrap().to_string(),
                "kick" => {
                    let user_opts = command.data.options.get(0).expect("Expected user option").resolved.as_ref().expect("Exected user object");
                    let reason_opts = command.data.options.get(1).expect("Expected reason option").resolved.as_ref().expect("Expected reason");

                    if let CommandDataOptionValue::User(user, _member) = user_opts {
                        if let CommandDataOptionValue::String(reason) = reason_opts {
                        
                        kick(guild, &ctx.http, user.id, reason).await.unwrap();

                        format!("Kicked user {} with reason: {}", user.tag(), reason)
                        } else {
                            "Please provide a reason".to_string()
                        } 
                    } else {
                        "Please provide a valid user".to_string()
                    }
                }
                _=> "Not added".to_string(),
            };

            if let Err(_err) = command.create_interaction_response(&ctx.http, |response| {
                response.kind(InteractionResponseType::ChannelMessageWithSource).interaction_response_data(|message| message.content(content))
            }).await
            {
                println!("Failed to respond. Error: {}", _err);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} bot is connected!", ready.user.name);

        let guild_id = GuildId(env::var("GUILD_ID").expect("Expected GUILD ID in env file").parse().expect("GUILD ID needs to be a int"));

        let _ = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command.name("ping").description("Who is daddy?")
            })
            .create_application_command(|command| {
                command.name("joke").description("Outputs a funny joke!")
            })
            .create_application_command(|command| {
                command.name("kick").description("Kicks a player").default_member_permissions(Permissions::KICK_MEMBERS)
                    .create_option(|option| {
                        option
                            .name("user")
                            .description("The user to ban")
                            .kind(CommandOptionType::User)
                            .required(true)
                    })
                    .create_option(|option| {
                        option
                            .name("reason")
                            .description("Reason for kicking")
                            .kind(CommandOptionType::String)
                            .required(true)
                    })
            })
        }).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client =
        Client::builder(&token, intents).event_handler(Handler).await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
