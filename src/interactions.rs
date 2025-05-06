use crate::database;
use crate::{bot::Bot, error};
use serenity::all::*;

impl Bot {
    pub async fn hanlde_interactions(
        &self,
        ctx: Context,
        interaction: Interaction,
    ) -> error::Result<()> {
        match interaction {
            Interaction::Command(command) => Ok(self.handle_command(ctx, command).await?),
            Interaction::Autocomplete(command) => {
                if command.data.name == "vote" {
                    let user_list = database::GuildUser::get_users_for_guild(
                        &self.pool,
                        command.guild_id.unwrap().into(),
                    )
                    .await?;

                    let choices: Vec<AutocompleteChoice> = user_list
                        .iter()
                        .map(|user| {
                            AutocompleteChoice::new(
                                user.username.clone(),
                                user.discord_user_id.to_string(),
                            )
                        })
                        .collect();

                    if let Err(e) = command
                        .create_response(
                            &ctx.http,
                            CreateInteractionResponse::Autocomplete(
                                CreateAutocompleteResponse::new().set_choices(choices),
                            ),
                        )
                        .await
                    {
                        println!("ya soucis {}", e);
                    }
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
