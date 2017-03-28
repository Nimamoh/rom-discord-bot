//https://discordapp.com/oauth2/authorize?&client_id=295890698417340417&scope=bot&permissions=0

/* Macros ****/
#![feature(conservative_impl_trait)]

extern crate discord;

extern crate futures;

use futures::Future;

use discord::Discord;
use discord::model::Event;

#[macro_use]
extern crate serde_derive;

mod vending;

use vending::VendingApi;
use vending::VendingApiError;
use vending::{ItemOnSale, BuyingItem};

use std::sync::Arc;

fn look_for_sale(vending_api: VendingApi, name: String) -> impl Future<Item=Vec<ItemOnSale>, Error=VendingApiError> {
    vending_api.vending()
        .map(move |items| {
            //Filter by item name. Ignore case
            let filtered: Vec<ItemOnSale> = items.into_iter().filter(move |sell_item| {
                let sane_selling_item_name = sell_item.name().to_lowercase();
                let sane_item_name = name.to_lowercase();
                let sane_item_name = sane_item_name.trim();

                sane_selling_item_name.contains(&sane_item_name)
            }).collect();
            filtered
        })
}

fn look_for_buying(vending_api: VendingApi, name: String) -> impl Future<Item=Vec<BuyingItem>, Error=VendingApiError> {
    vending_api.buying()
        .map(move |items| {
//            println!("items = {:?}", items);
            //Filter by item name. Ignore case
            let filtered: Vec<BuyingItem> = items.into_iter().filter(move |buying_item| {
                let sane_buying_item_name = buying_item.name().to_lowercase();
                let sane_item_name = name.to_lowercase();
                let sane_item_name = sane_item_name.trim();

//                println!("Compare {} with {}", sane_buying_item_name, sane_item_name);
                sane_buying_item_name.contains(&sane_item_name)
            }).collect();
            filtered
        })
}

fn main() {
    let token_bot = "Mjk1ODkwNjk4NDE3MzQwNDE3.C7qTWA.uNinGhRhSMj3sun83Ke_uIF4kjw";

    let discord = Discord::from_bot_token(&token_bot).expect("Unable to make bot from token.");
    let discord = Arc::new(discord);

    let vending_api = VendingApi::new();

    let (mut connection, _) = discord.connect().expect("Unable to connect.");

    println!("Ready.");


    loop {
        match connection.recv_event() {
            Ok(Event::MessageCreate(message)) => {
                let content = message.content.clone();
                if content.is_char_boundary(2) {
                    let (prefix, item) = content.split_at(2);

                    let item_name = item.to_string();
                    let vending_api = vending_api.clone();
                    let channel_id = message.channel_id.clone();
                    let discord = discord.clone();

                    match prefix {
                        "B>" => {
                            println!("Look for selling {}", item);

                            std::thread::spawn(move || {
                                let items = look_for_sale(vending_api, item_name).wait().unwrap();

                                if items.len() == 0 {
                                    discord.send_message(channel_id, "Sorry, the item you're looking for doesn't seem to be in sale", "", false);
                                }

                                for item in items.into_iter() {
                                    let msg : String = format!("**{}** **{}** for **{}**z each. Shop name: **{}**. **{}**", item.amount(), item.name(), item.price(), item.shop_name(), item.location());
                                    discord.send_message(channel_id, &msg, "", false);
                                }
                            });
                        }
                        "S>" => {
                            println!("Look for buying {}", item);

                            std::thread::spawn(move || {
                                let items = look_for_buying(vending_api, item_name).wait().unwrap();

                                if items.len() == 0 {
                                    discord.send_message(channel_id, "Sorry, the item you wanna sell doesn't seem to match any buy proposition right now", "", false);
                                }

                                for item in items.into_iter() {
                                    let msg : String = format!("**{}** **{}** for **{}**z each. Shop name: **{}**. **{}**", item.amount(), item.name(), item.price(), item.shop_name(), item.location());
                                    discord.send_message(channel_id, &msg, "", false);
                                }
                            });
                        }
                        _ => (),
                    }
                }

                println!("{} says: {}", message.author.name, message.content);
            }
            Ok(_) => {}
            Err(discord::Error::Closed(code, body)) => {
                println!("Gateway closed on us with code {:?}: {}", code, body);
                break
            }
            Err(err) => println!("Received error: {:?}", err)
        }
    }
}
