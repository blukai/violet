use std::{collections::HashSet, env};

#[macro_use]
extern crate serenity;
use serenity::{
    client::{Client, Context, EventHandler},
    framework::StandardFramework,
    http,
    model::{event::ResumedEvent, gateway::Ready},
};

mod commands;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("connected as {}", ready.user.name);
    }

    fn resume(&self, _: Context, _: ResumedEvent) {
        println!("resumed")
    }
}

fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("missing req'd env var DISCORD_TOKEN");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token, Handler).expect("could not create client");

    let owners = match http::get_current_application_info() {
        Ok(info) => {
            let mut set = HashSet::new();
            set.insert(info.owner.id);

            set
        }
        Err(why) => panic!("could not get app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix("!"))
        .command("ping", |c| c.cmd(commands::meta::ping))
        .command("pong", |c| c.cmd(commands::meta::pong))
        .command("clear", |c| c.cmd(commands::meta::clear).owners_only(true));

    client.with_framework(framework);

    // Start a single shard, and start listening to events.
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start() {
        println!("client error: {:?}", why);
    }
}
