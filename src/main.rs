//client id: 295890698417340417
//client secret: eeX4u7GhSXAvt4CquizWb_--5sY41oQP

//https://discordapp.com/oauth2/authorize?&client_id=295890698417340417&scope=bot&permissions=0

extern crate discord;

use discord::Discord;
use discord::model::Event;

fn main() {

   let token_bot = "Mjk1ODkwNjk4NDE3MzQwNDE3.C7qTWA.uNinGhRhSMj3sun83Ke_uIF4kjw";

   let discord = Discord::from_bot_token(&token_bot).expect("Unable to make bot from token.");


   let (mut connection, _) = discord.connect().expect("Unable to connect.");

   println!("Ready.");


   loop {

      match connection.recv_event() {
         Ok(Event::MessageCreate(message)) => {
            println!("{} says: {}", message.author.name, message.content);

            if message.content == "!quit" {
               println!("Quitting");
               break
            }
         }
         Ok(_) => { }
         Err(discord::Error::Closed(code, body)) => {
            println!("Gateway closed on us with code {:?}: {}", code, body);
            break
         }
         Err(err) => println!("Received error: {:?}", err)
      }
   }
}
